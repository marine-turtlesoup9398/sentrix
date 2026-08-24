use crate::model::{GraphNode, SoftwareKnowledgeGraph};
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Direction;
use sentrix_ir::{ConfidenceLevel, Evidence, EvidenceSourceType, EvidenceStrength};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct CentralityMetrics {
    pub node_id: String,
    pub in_degree: usize,
    pub out_degree: usize,
    pub pagerank_score: f32,
}

pub struct GraphQueryEngine;

impl GraphQueryEngine {
    pub fn find_node_index(graph: &SoftwareKnowledgeGraph, target_id: &str) -> Option<NodeIndex> {
        if let Some(&idx) = graph.node_map.get(target_id) {
            return Some(idx);
        }
        let file_key = format!("file:{}", target_id);
        if let Some(&idx) = graph.node_map.get(&file_key) {
            return Some(idx);
        }
        // Partial match fallback
        for (key, &idx) in &graph.node_map {
            if key.contains(target_id) {
                return Some(idx);
            }
        }
        None
    }

    pub fn get_direct_dependents(
        graph: &SoftwareKnowledgeGraph,
        target_id: &str,
    ) -> Vec<GraphNode> {
        let mut dependents = Vec::new();
        if let Some(node_idx) = Self::find_node_index(graph, target_id) {
            for neighbor in graph
                .graph
                .neighbors_directed(node_idx, Direction::Incoming)
            {
                dependents.push(graph.graph[neighbor].clone());
            }
        }
        dependents
    }

    pub fn get_transitive_downstream(
        graph: &SoftwareKnowledgeGraph,
        start_id: &str,
    ) -> Vec<GraphNode> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();

        if let Some(start_idx) = Self::find_node_index(graph, start_id) {
            let mut queue = VecDeque::new();
            queue.push_back(start_idx);
            visited.insert(start_idx);

            while let Some(curr) = queue.pop_front() {
                for neighbor in graph.graph.neighbors_directed(curr, Direction::Incoming) {
                    if visited.insert(neighbor) {
                        result.push(graph.graph[neighbor].clone());
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        result
    }

    pub fn find_cycles(graph: &SoftwareKnowledgeGraph) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        let mut on_stack = HashSet::new();

        for node_idx in graph.graph.node_indices() {
            if !visited.contains(&node_idx) {
                Self::dfs_cycles(
                    graph,
                    node_idx,
                    &mut visited,
                    &mut stack,
                    &mut on_stack,
                    &mut cycles,
                );
            }
        }

        cycles
    }

    fn dfs_cycles(
        graph: &SoftwareKnowledgeGraph,
        node: NodeIndex,
        visited: &mut HashSet<NodeIndex>,
        stack: &mut Vec<NodeIndex>,
        on_stack: &mut HashSet<NodeIndex>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        visited.insert(node);
        on_stack.insert(node);
        stack.push(node);

        for neighbor in graph.graph.neighbors_directed(node, Direction::Outgoing) {
            if !visited.contains(&neighbor) {
                Self::dfs_cycles(graph, neighbor, visited, stack, on_stack, cycles);
            } else if on_stack.contains(&neighbor) {
                let mut cycle_path = Vec::new();
                if let Some(pos) = stack.iter().position(|&x| x == neighbor) {
                    for &idx in &stack[pos..] {
                        cycle_path.push(graph.graph[idx].id.clone());
                    }
                    cycle_path.push(graph.graph[neighbor].id.clone());
                }
                cycles.push(cycle_path);
            }
        }

        stack.pop();
        on_stack.remove(&node);
    }

    pub fn calculate_centrality(
        graph: &SoftwareKnowledgeGraph,
    ) -> HashMap<String, CentralityMetrics> {
        let mut metrics = HashMap::new();
        let n = graph.graph.node_count();
        if n == 0 {
            return metrics;
        }

        let initial_pr = 1.0 / n as f32;
        let mut pagerank: HashMap<NodeIndex, f32> = graph
            .graph
            .node_indices()
            .map(|idx| (idx, initial_pr))
            .collect();

        let damping = 0.85;
        for _ in 0..10 {
            let mut new_pr = HashMap::new();
            for node_idx in graph.graph.node_indices() {
                let mut rank_sum = 0.0;
                for incoming in graph
                    .graph
                    .neighbors_directed(node_idx, Direction::Incoming)
                {
                    let out_deg = graph
                        .graph
                        .neighbors_directed(incoming, Direction::Outgoing)
                        .count();
                    if out_deg > 0 {
                        rank_sum += pagerank.get(&incoming).unwrap_or(&0.0) / out_deg as f32;
                    }
                }
                let node_pr = (1.0 - damping) / n as f32 + damping * rank_sum;
                new_pr.insert(node_idx, node_pr);
            }
            pagerank = new_pr;
        }

        for node_idx in graph.graph.node_indices() {
            let node = &graph.graph[node_idx];
            let in_deg = graph
                .graph
                .neighbors_directed(node_idx, Direction::Incoming)
                .count();
            let out_deg = graph
                .graph
                .neighbors_directed(node_idx, Direction::Outgoing)
                .count();
            let pr = *pagerank.get(&node_idx).unwrap_or(&0.0);

            metrics.insert(
                node.id.clone(),
                CentralityMetrics {
                    node_id: node.id.clone(),
                    in_degree: in_deg,
                    out_degree: out_deg,
                    pagerank_score: pr,
                },
            );
        }

        metrics
    }

    pub fn find_shortest_path(
        graph: &SoftwareKnowledgeGraph,
        start_id: &str,
        target_id: &str,
    ) -> Option<Vec<Evidence>> {
        let start_idx = Self::find_node_index(graph, start_id)?;
        let target_idx = Self::find_node_index(graph, target_id)?;

        let mut predecessors = HashMap::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back(start_idx);
        visited.insert(start_idx);

        let mut found = false;
        while let Some(curr) = queue.pop_front() {
            if curr == target_idx {
                found = true;
                break;
            }

            for edge in graph.graph.edges_directed(curr, Direction::Outgoing) {
                let next = edge.target();
                if visited.insert(next) {
                    predecessors.insert(next, (curr, edge.weight().clone()));
                    queue.push_back(next);
                }
            }
        }

        if !found {
            return None;
        }

        let mut path_evidence = Vec::new();
        let mut curr = target_idx;
        while let Some((prev, edge_data)) = predecessors.get(&curr) {
            let prev_node = &graph.graph[*prev];
            let curr_node = &graph.graph[curr];

            path_evidence.push(Evidence {
                id: format!("ev_path_{}_{}", prev_node.id, curr_node.id),
                source_type: EvidenceSourceType::CallGraph,
                file_path: curr_node.path.clone(),
                line: None,
                column: None,
                symbol: Some(curr_node.name.clone()),
                relationship: Some(format!("{:?}", edge_data.edge_type)),
                commit: None,
                description: format!(
                    "{} {:?} {}",
                    prev_node.name, edge_data.edge_type, curr_node.name
                ),
                strength: EvidenceStrength::DirectlyObserved,
                confidence: ConfidenceLevel::High,
                timestamp: chrono::Utc::now().to_rfc3339(),
            });

            curr = *prev;
        }

        path_evidence.reverse();
        Some(path_evidence)
    }
}
