use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use sentrix_ir::{ApiEndpointItem, DependencyItem, FileItem, SecurityFindingItem};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeType {
    Repository,
    Directory,
    File,
    Module,
    Class,
    Function,
    ApiEndpoint,
    Dependency,
    Finding,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub name: String,
    pub node_type: NodeType,
    pub path: Option<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EdgeType {
    Contains,
    Imports,
    DependsOn,
    Calls,
    Extends,
    Exposes,
    FlowsTo,
    Affects,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    pub edge_type: EdgeType,
    pub weight: f32,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SoftwareKnowledgeGraph {
    pub graph: DiGraph<GraphNode, GraphEdge>,
    pub node_map: HashMap<String, NodeIndex>,
}

impl Default for SoftwareKnowledgeGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl SoftwareKnowledgeGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            node_map: HashMap::new(),
        }
    }

    pub fn add_node(
        &mut self,
        id: String,
        name: String,
        node_type: NodeType,
        path: Option<String>,
    ) -> NodeIndex {
        if let Some(&idx) = self.node_map.get(&id) {
            return idx;
        }

        let node = GraphNode {
            id: id.clone(),
            name,
            node_type,
            path,
            metadata: HashMap::new(),
        };

        let idx = self.graph.add_node(node);
        self.node_map.insert(id, idx);
        idx
    }

    pub fn add_edge(&mut self, from_id: &str, to_id: &str, edge_type: EdgeType) {
        if let (Some(&from_idx), Some(&to_idx)) =
            (self.node_map.get(from_id), self.node_map.get(to_id))
        {
            self.graph.add_edge(
                from_idx,
                to_idx,
                GraphEdge {
                    edge_type,
                    weight: 1.0,
                    description: None,
                },
            );
        }
    }

    pub fn build_from_sir(
        &mut self,
        files: &[FileItem],
        apis: &[ApiEndpointItem],
        dependencies: &[DependencyItem],
        findings: &[SecurityFindingItem],
    ) {
        // Add root repository node
        let root_idx = self.add_node(
            "repo_root".to_string(),
            "Repository Root".to_string(),
            NodeType::Repository,
            None,
        );

        // Add Dependencies
        for dep in dependencies {
            let dep_id = format!("dep:{}", dep.name);
            let dep_idx =
                self.add_node(dep_id.clone(), dep.name.clone(), NodeType::Dependency, None);
            self.graph.add_edge(
                root_idx,
                dep_idx,
                GraphEdge {
                    edge_type: EdgeType::DependsOn,
                    weight: 1.0,
                    description: Some(format!("{} v{}", dep.ecosystem, dep.version)),
                },
            );
        }

        // Add Files & Symbols
        for file in files {
            let file_id = format!("file:{}", file.relative_path);
            let file_idx = self.add_node(
                file_id.clone(),
                file.relative_path.clone(),
                NodeType::File,
                Some(file.relative_path.clone()),
            );

            self.graph.add_edge(
                root_idx,
                file_idx,
                GraphEdge {
                    edge_type: EdgeType::Contains,
                    weight: 1.0,
                    description: None,
                },
            );

            // Add File Imports
            for imp in &file.imports {
                let imp_id = format!("import:{}", imp);
                let imp_idx = self.add_node(imp_id.clone(), imp.clone(), NodeType::Module, None);
                self.graph.add_edge(
                    file_idx,
                    imp_idx,
                    GraphEdge {
                        edge_type: EdgeType::Imports,
                        weight: 1.0,
                        description: None,
                    },
                );
            }

            // Add Classes
            for class in &file.classes {
                let class_id = format!("class:{}", class.id);
                let class_idx = self.add_node(
                    class_id.clone(),
                    class.name.clone(),
                    NodeType::Class,
                    Some(file.relative_path.clone()),
                );

                self.graph.add_edge(
                    file_idx,
                    class_idx,
                    GraphEdge {
                        edge_type: EdgeType::Contains,
                        weight: 1.0,
                        description: None,
                    },
                );
            }

            // Add Functions
            for func in &file.functions {
                let func_id = format!("func:{}", func.id);
                let func_idx = self.add_node(
                    func_id.clone(),
                    func.name.clone(),
                    NodeType::Function,
                    Some(file.relative_path.clone()),
                );

                self.graph.add_edge(
                    file_idx,
                    func_idx,
                    GraphEdge {
                        edge_type: EdgeType::Contains,
                        weight: 1.0,
                        description: None,
                    },
                );

                // Add Call relationships
                for callee in &func.calls {
                    let callee_id = format!("func:{}", callee);
                    let callee_idx =
                        self.add_node(callee_id.clone(), callee.clone(), NodeType::Function, None);
                    self.graph.add_edge(
                        func_idx,
                        callee_idx,
                        GraphEdge {
                            edge_type: EdgeType::Calls,
                            weight: 1.0,
                            description: None,
                        },
                    );
                }
            }
        }

        // Add APIs
        for api in apis {
            let api_id = format!("api:{}", api.id);
            let api_idx = self.add_node(
                api_id.clone(),
                format!("{} {}", api.http_method, api.path_pattern),
                NodeType::ApiEndpoint,
                None,
            );

            let handler_id = format!("func:{}", api.handler_symbol);
            if let Some(&handler_idx) = self.node_map.get(&handler_id) {
                self.graph.add_edge(
                    api_idx,
                    handler_idx,
                    GraphEdge {
                        edge_type: EdgeType::Exposes,
                        weight: 1.0,
                        description: None,
                    },
                );
            }
        }

        // Add Findings
        for finding in findings {
            let finding_id = format!("finding:{}", finding.id);
            let finding_idx = self.add_node(
                finding_id.clone(),
                finding.title.clone(),
                NodeType::Finding,
                Some(finding.location.file_path.to_string_lossy().to_string()),
            );

            let target_file_id = format!("file:{}", finding.location.file_path.to_string_lossy());
            if let Some(&file_idx) = self.node_map.get(&target_file_id) {
                self.graph.add_edge(
                    finding_idx,
                    file_idx,
                    GraphEdge {
                        edge_type: EdgeType::Affects,
                        weight: 1.0,
                        description: Some(finding.description.clone()),
                    },
                );
            }
        }
    }

    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn get_impact_radius(&self, changed_files: &[String]) -> Vec<GraphNode> {
        let mut affected = Vec::new();
        let mut visited = std::collections::HashSet::new();

        for file in changed_files {
            let file_key = format!("file:{}", file);
            if let Some(&start_idx) = self.node_map.get(&file_key) {
                let mut bfs = petgraph::visit::Bfs::new(&self.graph, start_idx);
                while let Some(nx) = bfs.next(&self.graph) {
                    if visited.insert(nx) {
                        affected.push(self.graph[nx].clone());
                    }
                }
            }
        }
        affected
    }

    pub fn export_json(&self) -> serde_json::Value {
        let nodes: Vec<serde_json::Value> = self
            .graph
            .node_indices()
            .map(|idx| {
                let n = &self.graph[idx];
                serde_json::json!({
                    "id": n.id,
                    "name": n.name,
                    "type": format!("{:?}", n.node_type),
                    "path": n.path
                })
            })
            .collect();

        let edges: Vec<serde_json::Value> = self
            .graph
            .edge_references()
            .map(|e| {
                let source = &self.graph[e.source()];
                let target = &self.graph[e.target()];
                serde_json::json!({
                    "source": source.id,
                    "target": target.id,
                    "type": format!("{:?}", e.weight().edge_type)
                })
            })
            .collect();

        serde_json::json!({
            "nodes": nodes,
            "edges": edges,
            "stats": {
                "node_count": self.node_count(),
                "edge_count": self.edge_count()
            }
        })
    }
}
