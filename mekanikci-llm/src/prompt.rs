pub struct PromptManager;

impl PromptManager {
    pub fn system_prompt() -> String {
        String::new()
    }

    pub fn build_prompt(_user_input: &str) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_prompt_returns_string() {
        let prompt = PromptManager::system_prompt();
        assert!(prompt.is_empty());
    }
}
