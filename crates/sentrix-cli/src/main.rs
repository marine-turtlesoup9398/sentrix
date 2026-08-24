use clap::{Parser, Subcommand};
use colored::*;
use std::path::{Path, PathBuf};
use std::time::Instant;

use sentrix_ai::AiProvider;
use sentrix_analysis::{
    ArchitectureDriftEngine, ArchitectureEngine, BenchmarkEngine, ComplexityEngine,
    DependencyIntelligenceEngine, HotspotEngine, RepositoryHealthEngine,
};
use sentrix_api::{ApiServer, AppState};
use sentrix_core::{telemetry::init_telemetry, SentrixConfig};
use sentrix_evolution::{
    CoChangeEngine, EvolutionGitExtractor, OwnershipEngine, PatternMiningEngine,
    PredictiveRiskEngine, SymbolHistoryEngine, TestRecommendationEngine,
};
use sentrix_git::GitIntelligence;
use sentrix_graph::SoftwareKnowledgeGraph;
use sentrix_impact::ImpactEngine;
use sentrix_ir::{FileItem, SecurityFindingItem};
use sentrix_parser::CodeExtractor;
use sentrix_search::{QueryIntentEngine, SearchEngine};
use sentrix_security::{DataFlowEngine, SbomComponent, SbomGenerator, SecretScanner};

#[derive(Parser)]
#[command(name = "sentrix")]
#[command(about = "SENTRIX — Software Intelligence & Engineering Risk Explorer", long_about = None)]
struct Cli {
    #[arg(short, long, global = true)]
    json: bool,

    #[arg(short, long, global = true)]
    verbose: bool,

    #[arg(long, global = true)]
    no_ai: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze repository code, dependencies, history & security
    Analyze {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Run coverage diagnostics breakdown on graph entities and languages
    Diagnostics {
        #[command(subcommand)]
        subcommand: DiagnosticsSubcommand,
    },
    /// Run automated performance & accuracy benchmark on local path or git URL
    Benchmark {
        #[arg(default_value = ".")]
        target: String,
    },
    /// Show repository health scores across 6 categories
    Health {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Show engineering risk hotspots & risk breakdown
    Risk {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Detect architecture layer rule violations & drift
    Drift {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Run dependency intelligence & circular dependency detection
    Dependency {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Calculate blast radius for specific package or component
    DependencyImpact {
        target: String,
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Show analysis status & repository health metrics
    Status {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Export or inspect Software Knowledge Graph
    Graph {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Perform fast symbol and entity search across repository
    Search {
        query: String,
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Execute intent-driven grounded graph query
    Query {
        query: String,
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Calculate change impact radius for working tree, commit range, or file
    Impact {
        #[arg(default_value = "HEAD~1..HEAD")]
        revision_or_file: String,
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// List engineering hotspots & high-risk components
    Hotspots {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Inspect security surface, secrets & data flows
    Security {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short, long, default_value = "table")]
        format: String, // table, json, sarif
    },
    /// Discover system architecture & component boundaries
    Architecture {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Query repository using evidence-backed AI explanation
    Ask {
        question: String,
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Launch local web GUI server
    Serve {
        #[arg(default_value = ".")]
        path: PathBuf,
        #[arg(short, long, default_value = "7070")]
        port: u16,
    },
    /// Query historical symbol evolution from Git commits
    History {
        #[arg(short, long)]
        symbol: Option<String>,
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Show repository historical change patterns and co-changes
    Evolution {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Estimate predictive change risk for component or diff
    Predict {
        target: String,
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Show contribution concentration & bus factor for component
    Ownership {
        target: String,
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Export CycloneDX / SPDX Software Bill of Materials (SBOM) JSON
    Sbom {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Configuration management and validation
    Config {
        #[command(subcommand)]
        subcommand: ConfigSubcommand,
    },
}

#[derive(Subcommand)]
enum DiagnosticsSubcommand {
    /// Detailed entity, relationship, and parser coverage audit
    Coverage {
        #[arg(default_value = ".")]
        path: PathBuf,
    },
}

#[derive(Subcommand)]
enum ConfigSubcommand {
    /// Validate sentrix.toml configuration file schema and bounds
    Validate {
        #[arg(default_value = "sentrix.toml")]
        path: PathBuf,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    init_telemetry(cli.verbose);

    match cli.command {
        Commands::Analyze { path } => {
            run_analysis(&path, cli.json).await?;
        }
        Commands::Diagnostics { subcommand } => match subcommand {
            DiagnosticsSubcommand::Coverage { path } => {
                let (files, graph, _, _, comp) = scan_repository(&path)?;
                if cli.json {
                    println!(
                        "{}",
                        serde_json::json!({
                            "files": files.len(),
                            "loc": comp.total_lines,
                            "functions": comp.total_functions,
                            "nodes": graph.node_count(),
                            "edges": graph.edge_count(),
                        })
                    );
                } else {
                    println!("\n--- SENTRIX DIAGNOSTICS & COVERAGE AUDIT ---");
                    println!("Total Files Scanned:       {}", files.len());
                    println!("Total Lines of Code:       {}", comp.total_lines);
                    println!("Total Functions Extracted: {}", comp.total_functions);
                    println!(
                        "Knowledge Graph:           {} nodes, {} edges",
                        graph.node_count(),
                        graph.edge_count()
                    );
                }
            }
        },
        Commands::Config { subcommand } => match subcommand {
            ConfigSubcommand::Validate { path } => {
                let config = SentrixConfig::load_from_file(&path).unwrap_or_default();
                let errors = config.validate();

                if errors.is_empty() {
                    if cli.json {
                        println!("{}", serde_json::json!({ "valid": true, "errors": [] }));
                    } else {
                        println!(
                            "{}",
                            format!("✓ Configuration file '{}' is valid.", path.display())
                                .green()
                                .bold()
                        );
                    }
                } else {
                    if cli.json {
                        println!(
                            "{}",
                            serde_json::json!({ "valid": false, "errors": errors })
                        );
                    } else {
                        println!(
                            "{}",
                            format!(
                                "✗ Configuration validation failed for '{}':",
                                path.display()
                            )
                            .red()
                            .bold()
                        );
                        for err in &errors {
                            println!("  • Error: {}", err);
                        }
                    }
                    std::process::exit(2);
                }
            }
        },
        Commands::Benchmark { target } => {
            let report = BenchmarkEngine::run_benchmark(&target)?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!("\n--- BENCHMARK REPORT COMPLETED ---");
                println!("Files Analyzed:          {}", report.total_files);
                println!("Lines of Code:           {}", report.lines_of_code);
                println!("Functions Extracted:     {}", report.total_functions);
                println!(
                    "Knowledge Graph:         {} nodes, {} edges",
                    report.graph_nodes, report.graph_edges
                );
                println!("Architecture Pattern:    {:?}", report.architecture_pattern);
            }
        }
        Commands::Health { path } => {
            let (files, _, findings, hotspots, _) = scan_repository(&path)?;
            let arch = ArchitectureEngine::discover(&files);
            let health =
                RepositoryHealthEngine::compute_health(&files, &findings, &hotspots, &arch);
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&health)?);
            } else {
                println!(
                    "\n--- REPOSITORY HEALTH SCORECARD: {} / 100 ---",
                    health.overall_score
                );
                println!(
                    "• Architecture Score:  {}/100",
                    health.architecture_score.score
                );
                println!("• Security Score:      {}/100", health.security_score.score);
                println!(
                    "• Maintainability:     {}/100",
                    health.maintainability_score.score
                );
            }
        }
        Commands::Risk { path } => {
            let (_files, _, _, hotspots, _) = scan_repository(&path)?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&hotspots)?);
            } else {
                println!("\n--- RISK HOTSPOTS & ENGINEERING RISK ---");
                for h in hotspots.iter().take(5) {
                    println!(
                        "• {} - Risk Score: {:.1} [{:?}]",
                        h.relative_path, h.hotspot_score, h.risk_level
                    );
                }
            }
        }
        Commands::Drift { path } => {
            let (files, graph, _, _, _) = scan_repository(&path)?;
            let config = SentrixConfig::load_from_file(&path).unwrap_or_default();
            let drift =
                ArchitectureDriftEngine::analyze_drift(&graph, &files, &config.architecture.rules);
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&drift)?);
            } else {
                println!("\n--- ARCHITECTURE DRIFT & VIOLATION REPORT ---");
                println!("Total Violations: {}", drift.violations_count);
            }
            if drift.violations_count > 0 {
                std::process::exit(1);
            }
        }
        Commands::Dependency { path } => {
            let (_, graph, _, _, _) = scan_repository(&path)?;
            let report = DependencyIntelligenceEngine::detect_circular_dependencies(&graph);
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!("\n--- DEPENDENCY INTELLIGENCE REPORT ---");
                println!("Circular Dependency Cycles: {}", report.total_cycles_found);
            }
        }
        Commands::DependencyImpact { target, path } => {
            let (files, graph, _, _, _) = scan_repository(&path)?;
            let report =
                DependencyIntelligenceEngine::calculate_blast_radius(&graph, &files, &[], &target);
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!("\n--- DEPENDENCY BLAST RADIUS FOR '{}' ---", target);
                println!("Blast Radius Level: {:?}", report.blast_radius_level);
            }
        }
        Commands::Status { path } => {
            run_analysis(&path, cli.json).await?;
        }
        Commands::Graph { path } => {
            let (_, graph, _, _, _) = scan_repository(&path)?;
            if cli.json {
                println!(
                    "{}",
                    serde_json::json!({
                        "nodes": graph.node_count(),
                        "edges": graph.edge_count()
                    })
                );
            } else {
                println!(
                    "Knowledge Graph: {} nodes, {} edges",
                    graph.node_count(),
                    graph.edge_count()
                );
            }
        }
        Commands::Search { query, path } => {
            let (files, graph, _, _, _) = scan_repository(&path)?;
            let results = SearchEngine::search(&query, &files, &graph);
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&results)?);
            } else {
                println!("Found {} search results for '{}':", results.len(), query);
                for r in results.iter().take(10) {
                    println!("- [{}] {} @ {}", r.category, r.title, r.location);
                }
            }
        }
        Commands::Query { query, path } => {
            let (files, graph, _, _, _) = scan_repository(&path)?;
            let res = QueryIntentEngine::execute_grounded_query(&query, &files, &graph);
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&res)?);
            } else {
                println!("\n--- GROUNDED QUERY RESULT ---");
                println!("Intent:  {:?}", res.intent);
                println!("Answer:  {}", res.answer_summary);
            }
        }
        Commands::Impact {
            revision_or_file,
            path,
        } => {
            let (files, graph, _, _, _) = scan_repository(&path)?;
            let report = ImpactEngine::analyze_impact(
                &graph,
                &files,
                std::slice::from_ref(&revision_or_file),
                &revision_or_file,
            )?;
            let recs =
                TestRecommendationEngine::recommend_tests(&[revision_or_file], &files, &graph);

            if cli.json {
                println!(
                    "{}",
                    serde_json::json!({
                        "impact": report,
                        "recommended_tests": recs
                    })
                );
            } else {
                println!("\n--- CHANGE IMPACT & TEST RECOMMENDATION ---");
                println!(
                    "Directly Affected Components:   {}",
                    report.direct_dependents.len()
                );
                println!(
                    "Transitively Affected:          {}",
                    report.transitive_downstream.len()
                );
                println!("Recommended Tests to Re-run:    {}", recs.len());
                for r in recs {
                    println!("  • {} [{:?}]", r.test_file, r.priority);
                }
            }
        }
        Commands::Hotspots { path } => {
            let (_files, _, _, hotspots, _) = scan_repository(&path)?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&hotspots)?);
            } else {
                println!("Hotspots Count: {}", hotspots.len());
            }
        }
        Commands::Security { path, format: _ } => {
            let (_files, _, findings, _, _) = scan_repository(&path)?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&findings)?);
            } else {
                println!("Security Findings: {}", findings.len());
            }
        }
        Commands::Architecture { path } => {
            let (files, _, _, _, _) = scan_repository(&path)?;
            let arch = ArchitectureEngine::discover(&files);
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&arch)?);
            } else {
                println!("\n--- DISCOVERED SYSTEM ARCHITECTURE ---");
                println!("Pattern:            {:?}", arch.pattern);
                println!("Confidence:         {:.0}%", arch.confidence * 100.0);
            }
        }
        Commands::Ask { question, path } => {
            let (files, graph, _, _, _) = scan_repository(&path)?;
            let config = SentrixConfig::load_from_file(&path).unwrap_or_default();
            let grounded = QueryIntentEngine::execute_grounded_query(&question, &files, &graph);
            let provider = AiProvider::new(&config.ai.provider, config.ai.api_key.clone());
            let ans = provider.ask_grounded(&question, &grounded.evidence).await?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&ans)?);
            } else {
                println!("\n--- SENTRIX GROUNDED REASONING ---");
                println!("Answer: {}\n", ans.answer);
                println!("Confidence: {:?}", ans.confidence);
            }
        }
        Commands::Serve { path, port } => {
            let (files, graph, findings, hotspots, comp) = scan_repository(&path)?;
            let arch = ArchitectureEngine::discover(&files);
            let config = SentrixConfig::load_from_file(&path).unwrap_or_default();

            let state = AppState {
                files,
                graph,
                architecture: arch,
                hotspots,
                complexity: comp,
                findings,
                config,
            };

            let server = ApiServer::new(
                state,
                "127.0.0.1",
                port,
                Some(PathBuf::from("frontend/dist")),
            );
            server.run().await?;
        }
        Commands::History { symbol, path } => {
            let records =
                EvolutionGitExtractor::extract_commit_records(&path.to_string_lossy(), 100);
            if let Some(sym) = symbol {
                let report = SymbolHistoryEngine::query_symbol_history(&sym, None, &records);
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(&report)?);
                } else if let Some(r) = report {
                    println!("\n--- SYMBOL EVOLUTION HISTORY FOR '{}' ---", sym);
                    println!("Total Commits:      {}", r.total_commits);
                    println!("Unique Authors:     {}", r.unique_authors_count);
                    println!("Bugfix Commits:     {}", r.bugfix_associated_commits);
                } else {
                    println!("Historical evidence unavailable for symbol '{}'", sym);
                }
            } else if cli.json {
                println!("{}", serde_json::to_string_pretty(&records)?);
            } else {
                println!("Extracted {} historical commit records.", records.len());
            }
        }
        Commands::Evolution { path } => {
            let records =
                EvolutionGitExtractor::extract_commit_records(&path.to_string_lossy(), 100);
            let summary = EvolutionGitExtractor::summarize(&records);
            let co_changes = CoChangeEngine::mine_co_changes(&records, 2);
            let patterns = PatternMiningEngine::mine_patterns(&records, 3);

            if cli.json {
                println!(
                    "{}",
                    serde_json::json!({
                        "summary": summary,
                        "co_changes": co_changes,
                        "patterns": patterns
                    })
                );
            } else {
                println!("\n--- REPOSITORY EVOLUTION SUMMARY ---");
                println!("Total Commits Tracked: {}", summary.total_commits);
                println!("Unique Contributors:   {}", summary.total_authors);
                println!("Co-change Relationships: {}", co_changes.len());
                println!("Mined Change Patterns:   {}", patterns.len());
            }
        }
        Commands::Predict { target, path } => {
            let (files, graph, _, _, _) = scan_repository(&path)?;
            let report = PredictiveRiskEngine::predict_change_risk(&target, &files, &graph, 0, 0);
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!("\n--- PREDICTIVE CHANGE RISK FOR '{}' ---", target);
                println!("Predicted Risk:  {:?}", report.predicted_risk);
                println!("Confidence:      {:?}", report.confidence);
                println!("Limitations:     {}", report.limitations);
            }
        }
        Commands::Ownership { target, path } => {
            let records =
                EvolutionGitExtractor::extract_commit_records(&path.to_string_lossy(), 100);
            let ownership = OwnershipEngine::analyze_ownership(&target, &records);
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&ownership)?);
            } else {
                println!(
                    "\n--- HISTORICAL CONTRIBUTION CONCENTRATION FOR '{}' ---",
                    target
                );
                println!("Total Commits:                {}", ownership.total_commits);
                println!(
                    "Contribution Concentration:   {}",
                    ownership.contribution_concentration
                );
                println!("Interpretation:              {}", ownership.interpretation);
            }
        }
        Commands::Sbom { path } => {
            let (files, _, _, _, _) = scan_repository(&path)?;
            let mut components = Vec::new();
            for f in &files {
                components.push(SbomComponent {
                    name: f.relative_path.clone(),
                    version: "0.1.0".to_string(),
                    ecosystem: format!("{:?}", f.language),
                    license: Some("MIT".to_string()),
                    purl: None,
                });
            }
            let sbom = SbomGenerator::generate_spdx_sbom(&components);
            println!("{}", serde_json::to_string_pretty(&sbom)?);
        }
    }

    Ok(())
}

#[allow(clippy::type_complexity)]
fn scan_repository(
    path: &Path,
) -> anyhow::Result<(
    Vec<FileItem>,
    SoftwareKnowledgeGraph,
    Vec<SecurityFindingItem>,
    Vec<sentrix_analysis::HotspotItem>,
    sentrix_analysis::ComplexityMetricsSummary,
)> {
    let mut files = Vec::new();
    let mut all_apis = Vec::new();
    let mut all_findings = Vec::new();

    let walker = ignore::WalkBuilder::new(path)
        .hidden(false)
        .git_ignore(true)
        .filter_entry(|entry| {
            let path_str = entry.path().to_string_lossy();
            !path_str.contains("/target/")
                && !path_str.contains("/node_modules/")
                && !path_str.contains("/.git/")
                && !path_str.contains("/dist/")
        })
        .build();

    for entry in walker.flatten() {
        if entry.file_type().is_some_and(|ft| ft.is_file()) {
            let p = entry.path();
            if let Ok((file_item, apis)) = CodeExtractor::parse_file(p, path) {
                if file_item.language != sentrix_ir::Language::Unknown("binary".to_string())
                    && file_item.line_count > 0
                {
                    let content = std::fs::read_to_string(p).unwrap_or_default();
                    let sec_findings = SecretScanner::scan(&file_item, &content);
                    let (df_findings, _) = DataFlowEngine::analyze_file(&file_item, &content);

                    all_findings.extend(sec_findings);
                    all_findings.extend(df_findings);
                    all_apis.extend(apis);
                    files.push(file_item);
                }
            }
        }
    }

    let git = GitIntelligence::new(path);
    let git_metrics = git.analyze_history(200).unwrap_or_default();

    let mut graph = SoftwareKnowledgeGraph::new();
    graph.build_from_sir(&files, &all_apis, &[], &all_findings);

    let hotspots = HotspotEngine::compute_hotspots(&files, &git_metrics);
    let comp = ComplexityEngine::summarize(&files);

    Ok((files, graph, all_findings, hotspots, comp))
}

async fn run_analysis(path: &Path, json_output: bool) -> anyhow::Result<()> {
    let start = Instant::now();
    println!(
        "{}",
        "SENTRIX Software Intelligence Engine — Scanning Repository..."
            .bold()
            .cyan()
    );

    let (files, graph, findings, hotspots, comp) = scan_repository(path)?;
    let arch = ArchitectureEngine::discover(&files);
    let elapsed = start.elapsed();

    if json_output {
        println!(
            "{}",
            serde_json::json!({
                "files_count": files.len(),
                "lines_of_code": comp.total_lines,
                "architecture": arch,
                "hotspots": hotspots,
                "security_findings": findings,
                "graph_summary": {
                    "nodes": graph.node_count(),
                    "edges": graph.edge_count()
                },
                "elapsed_ms": elapsed.as_millis()
            })
        );
    } else {
        println!(
            "{}",
            format!("\nAnalysis completed in {:.2?}", elapsed)
                .green()
                .bold()
        );
        println!("------------------------------------------------------");
        println!("Files Analyzed:      {}", files.len().to_string().bold());
        println!(
            "Lines of Code:       {}",
            comp.total_lines.to_string().bold()
        );
        println!(
            "Functions Scanned:   {}",
            comp.total_functions.to_string().bold().green()
        );
        println!(
            "Knowledge Graph:     {} nodes, {} edges",
            graph.node_count(),
            graph.edge_count()
        );
        println!("Architecture:        {:?}", arch.pattern);
        println!(
            "Critical Hotspots:   {}",
            hotspots
                .iter()
                .filter(|h| h.risk_level == sentrix_analysis::RiskLevel::Critical)
                .count()
                .to_string()
                .red()
        );
        println!(
            "Security Findings:   {}",
            findings.len().to_string().yellow()
        );
        println!("------------------------------------------------------");
        println!(
            "{}",
            "Run `sentrix serve` to launch interactive Web GUI dashboard.".dimmed()
        );
    }

    Ok(())
}
