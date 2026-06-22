#[derive(Debug, Clone)]
pub struct SessionEntry {
    pub prompt: String,
    pub response: String,
}

#[derive(Debug, Clone)]
pub struct Session {
    pub entries: Vec<SessionEntry>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, prompt: String, response: String) {
        self.entries.push(SessionEntry { prompt, response });
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_new_is_empty() {
        let session = Session::new();
        assert!(session.entries.is_empty());
    }

    #[test]
    fn test_session_add_entry() {
        let mut session = Session::new();
        session.add_entry("prompt".into(), "response".into());
        assert_eq!(session.entries.len(), 1);
        assert_eq!(session.entries[0].prompt, "prompt");
        assert_eq!(session.entries[0].response, "response");
    }

    #[test]
    fn test_session_append_multiple() {
        let mut session = Session::new();
        session.add_entry("a".into(), "1".into());
        session.add_entry("b".into(), "2".into());
        assert_eq!(session.entries.len(), 2);
    }
}
