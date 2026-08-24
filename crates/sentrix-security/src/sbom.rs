use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SbomComponent {
    pub name: String,
    pub version: String,
    pub ecosystem: String,
    pub license: Option<String>,
    pub purl: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SbomDocument {
    pub bom_format: String,
    pub spec_version: String,
    pub serial_number: String,
    pub timestamp: String,
    pub components: Vec<SbomComponent>,
    pub license_audit_warning: String,
}

pub struct SbomGenerator;

impl SbomGenerator {
    pub fn generate_spdx_sbom(components: &[SbomComponent]) -> SbomDocument {
        SbomDocument {
            bom_format: "CycloneDX".to_string(),
            spec_version: "1.5".to_string(),
            serial_number: format!("urn:uuid:{}", uuid_simple()),
            timestamp: chrono::Utc::now().to_rfc3339(),
            components: components.to_vec(),
            license_audit_warning: "License metadata extracted automatically from package manifests requires legal review.".to_string(),
        }
    }
}

fn uuid_simple() -> String {
    format!(
        "{:x}",
        chrono::Utc::now()
            .timestamp_nanos_opt()
            .unwrap_or(123456789)
    )
}
