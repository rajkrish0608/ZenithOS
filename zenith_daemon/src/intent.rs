use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum IntentState {
    Coding,
    Writing,
    Browsing,
    Communication,
    Idle,
    Unknown,
}

pub struct IntentEngine {
    rules: HashMap<String, IntentState>,
}

impl IntentEngine {
    pub fn new() -> Self {
        let mut rules = HashMap::new();
        // Coding
        rules.insert("Code".to_string(), IntentState::Coding);
        rules.insert("cargo".to_string(), IntentState::Coding);
        rules.insert("rustc".to_string(), IntentState::Coding);
        rules.insert("Terminal".to_string(), IntentState::Coding);
        rules.insert("iTerm2".to_string(), IntentState::Coding);
        
        // Writing
        rules.insert("Word".to_string(), IntentState::Writing);
        rules.insert("Notion".to_string(), IntentState::Writing);
        rules.insert("Obsidian".to_string(), IntentState::Writing);
        rules.insert("TextEdit".to_string(), IntentState::Writing);

        // Browsing
        rules.insert("Google Chrome".to_string(), IntentState::Browsing);
        rules.insert("Safari".to_string(), IntentState::Browsing);
        rules.insert("Firefox".to_string(), IntentState::Browsing);

        // Comm
        rules.insert("Slack".to_string(), IntentState::Communication);
        rules.insert("Discord".to_string(), IntentState::Communication);
        rules.insert("Zoom".to_string(), IntentState::Communication);
        rules.insert("Microsoft Teams".to_string(), IntentState::Communication);

        // Debug/Verification Rule
        rules.insert("Antigravity".to_string(), IntentState::Coding);

        Self { rules }
    }

    pub fn classify(&self, app_name: &str) -> IntentState {
        // V1.1: Partial matching (e.g., "Code Helper" -> Coding)
        for (key, state) in &self.rules {
            if app_name.contains(key) {
                return state.clone();
            }
        }
        IntentState::Unknown
    }
}
