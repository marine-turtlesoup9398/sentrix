use sentrix_ir::FileItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityMetricsSummary {
    pub total_files: usize,
    pub total_lines: usize,
    pub total_functions: usize,
    pub avg_complexity: f32,
    pub max_complexity: u32,
    pub complex_functions_count: usize,
}

pub struct ComplexityEngine;

impl ComplexityEngine {
    pub fn summarize(files: &[FileItem]) -> ComplexityMetricsSummary {
        let total_files = files.len();
        let total_lines: usize = files.iter().map(|f| f.line_count).sum();

        let mut total_functions = 0;
        let mut total_comp = 0u32;
        let mut max_comp = 0u32;
        let mut complex_count = 0;

        for file in files {
            for func in &file.functions {
                total_functions += 1;
                total_comp += func.cyclomatic_complexity;
                if func.cyclomatic_complexity > max_comp {
                    max_comp = func.cyclomatic_complexity;
                }
                if func.cyclomatic_complexity > 5 {
                    complex_count += 1;
                }
            }
        }

        let avg_complexity = if total_functions > 0 {
            total_comp as f32 / total_functions as f32
        } else {
            0.0
        };

        ComplexityMetricsSummary {
            total_files,
            total_lines,
            total_functions,
            avg_complexity,
            max_complexity: max_comp,
            complex_functions_count: complex_count,
        }
    }
}
