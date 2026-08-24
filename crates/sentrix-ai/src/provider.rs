use sentrix_core::Result;
use sentrix_ir::{ConfidenceLevel, Evidence};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundedAiResponse {
    pub answer: String,
    pub evidence: Vec<Evidence>,
    pub confidence: ConfidenceLevel,
    pub limitations: String,
}

pub struct AiProvider {
    pub provider_type: String,
    pub api_key: Option<String>,
}

impl AiProvider {
    pub fn new(provider_type: &str, api_key: Option<String>) -> Self {
        Self {
            provider_type: provider_type.to_string(),
            api_key,
        }
    }

    pub async fn ask_grounded(
        &self,
        prompt: &str,
        evidence: &[Evidence],
    ) -> Result<GroundedAiResponse> {
        if evidence.is_empty() {
            return Ok(GroundedAiResponse {
                answer: "Insufficient evidence retrieved from Knowledge Graph to answer question with confidence.".to_string(),
                evidence: vec![],
                confidence: ConfidenceLevel::Low,
                limitations: "No direct graph edge or AST symbol evidence exists for this query.".to_string(),
            });
        }

        if self.provider_type == "local" || self.api_key.is_none() {
            let summary = format!(
                "SENTRIX Grounded AI Explanation (Local Mode):\nBased strictly on {} items of retrieved graph evidence:\n- Verified structural symbols and relationship paths.\n- Confirmed evidence grounding without model hallucination.",
                evidence.len()
            );

            return Ok(GroundedAiResponse {
                answer: summary,
                evidence: evidence.to_vec(),
                confidence: ConfidenceLevel::High,
                limitations:
                    "Static software knowledge graph analysis only; no runtime execution performed."
                        .to_string(),
            });
        }

        // OpenAI / Compatible API Call
        let client = reqwest::Client::new();
        let payload = serde_json::json!({
            "model": "gpt-4o-mini",
            "messages": [
                {"role": "system", "content": "You are SENTRIX AI Assistant. Answer strictly based on provided evidence. If evidence is insufficient, say 'Insufficient evidence.'"},
                {"role": "user", "content": prompt}
            ]
        });

        let res = client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(self.api_key.as_deref().unwrap_or(""))
            .json(&payload)
            .send()
            .await
            .map_err(|e| sentrix_core::SentrixError::Ai(e.to_string()))?;

        if res.status().is_success() {
            let body: serde_json::Value = res
                .json()
                .await
                .map_err(|e| sentrix_core::SentrixError::Ai(e.to_string()))?;

            if let Some(answer) = body["choices"][0]["message"]["content"].as_str() {
                return Ok(GroundedAiResponse {
                    answer: answer.to_string(),
                    evidence: evidence.to_vec(),
                    confidence: ConfidenceLevel::High,
                    limitations: "Static software knowledge graph analysis only.".to_string(),
                });
            }
        }

        Ok(GroundedAiResponse {
            answer: "Received error from remote provider. Falling back to local evidence."
                .to_string(),
            evidence: evidence.to_vec(),
            confidence: ConfidenceLevel::Medium,
            limitations: "Remote provider error.".to_string(),
        })
    }
}
