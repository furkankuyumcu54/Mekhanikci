use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize)]
struct GenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
    options: serde_json::Value,
}

#[derive(Deserialize)]
struct GenerateResponse {
    response: String,
}

pub struct OllamaClient {
    pub base_url: String,
    pub model: String,
    pub temperature: f64,
}

impl OllamaClient {
    pub fn new(base_url: impl Into<String>, model: impl Into<String>, temperature: f64) -> Self {
        Self {
            base_url: base_url.into(),
            model: model.into(),
            temperature,
        }
    }

    pub fn generate(&self, prompt: &str) -> anyhow::Result<String> {
        let url = format!("{}/api/generate", self.base_url.trim_end_matches('/'));

        let body = GenerateRequest {
            model: &self.model,
            prompt,
            stream: false,
            options: json!({ "temperature": self.temperature }),
        };

        let client = reqwest::blocking::Client::builder()
            .danger_accept_invalid_certs(false)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create HTTP client: {e}"))?;
        let resp = client
            .post(&url)
            .json(&body)
            .timeout(std::time::Duration::from_secs(120))
            .send()
            .map_err(|e| anyhow::anyhow!("Cannot connect to Ollama at {}: {}", url, e))?;

        let status = resp.status();
        if !status.is_success() {
            anyhow::bail!("Ollama returned HTTP {status}");
        }

        let raw_body = resp
            .text()
            .map_err(|e| anyhow::anyhow!("Failed to read Ollama response body: {e}"))?;

        let data: GenerateResponse = serde_json::from_str(&raw_body)
            .map_err(|e| anyhow::anyhow!("Failed to parse Ollama response: {e}"))?;

        if data.response.trim().is_empty() {
            anyhow::bail!("Ollama returned an empty response. Check that the model '{}' is available.", self.model);
        }

        Ok(data.response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ollama_client_new() {
        let client = OllamaClient::new("http://localhost:11434", "qwen3.5:4b", 0.0);
        assert_eq!(client.base_url, "http://localhost:11434");
        assert_eq!(client.model, "qwen3.5:4b");
        assert_eq!(client.temperature, 0.0);
    }
}
