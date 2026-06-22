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

    pub async fn generate(&self, _prompt: &str) -> anyhow::Result<String> {
        anyhow::bail!("OllamaClient::generate not implemented yet")
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
