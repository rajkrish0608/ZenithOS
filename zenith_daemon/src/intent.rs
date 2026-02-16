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

struct KeywordWeight {
    intent: IntentState,
    weight: f32, // 0.0 to 1.0
}

pub struct IntentEngine {
    // Map keyword -> (Intent, Weight)
    // e.g., "code" -> (Coding, 0.8)
    knowledge_base: Vec<(String, KeywordWeight)>,
}

impl IntentEngine {
    pub fn new() -> Self {
        let mut kb = Vec::new();

        // Coding Pattern
        kb.push(("Code".to_string(), KeywordWeight { intent: IntentState::Coding, weight: 0.9 }));
        kb.push(("cargo".to_string(), KeywordWeight { intent: IntentState::Coding, weight: 1.0 }));
        kb.push(("rustc".to_string(), KeywordWeight { intent: IntentState::Coding, weight: 1.0 }));
        kb.push(("Term".to_string(), KeywordWeight { intent: IntentState::Coding, weight: 0.7 })); // Terminal, iTerm
        kb.push(("Git".to_string(), KeywordWeight { intent: IntentState::Coding, weight: 0.8 }));

        // Writing Pattern
        kb.push(("Word".to_string(), KeywordWeight { intent: IntentState::Writing, weight: 0.9 }));
        kb.push(("Notion".to_string(), KeywordWeight { intent: IntentState::Writing, weight: 0.9 }));
        kb.push(("Obsidian".to_string(), KeywordWeight { intent: IntentState::Writing, weight: 0.9 }));
        kb.push(("Text".to_string(), KeywordWeight { intent: IntentState::Writing, weight: 0.6 }));

        // Browsing Pattern
        kb.push(("Chrome".to_string(), KeywordWeight { intent: IntentState::Browsing, weight: 0.8 }));
        kb.push(("Safari".to_string(), KeywordWeight { intent: IntentState::Browsing, weight: 0.8 }));
        kb.push(("Firefox".to_string(), KeywordWeight { intent: IntentState::Browsing, weight: 0.8 }));
        kb.push(("Brave".to_string(), KeywordWeight { intent: IntentState::Browsing, weight: 0.8 }));

        // Communication Pattern
        kb.push(("Slack".to_string(), KeywordWeight { intent: IntentState::Communication, weight: 0.9 }));
        kb.push(("Discord".to_string(), KeywordWeight { intent: IntentState::Communication, weight: 0.9 }));
        kb.push(("Teams".to_string(), KeywordWeight { intent: IntentState::Communication, weight: 0.9 }));
        kb.push(("Zoom".to_string(), KeywordWeight { intent: IntentState::Communication, weight: 0.9 }));

        Self { knowledge_base: kb }
    }

    /// Infers intent based on weighted keyword matching.
    /// Returns (Intent, Confidence Score)
    pub fn infer(&self, app_name: &str) -> (IntentState, f32) {
        let mut best_intent = IntentState::Unknown;
        let mut max_score = 0.0;

        for (keyword, weight_obj) in &self.knowledge_base {
            if app_name.contains(keyword) {
                // If match found, use its weight as score
                if weight_obj.weight > max_score {
                    max_score = weight_obj.weight;
                    best_intent = weight_obj.intent.clone();
                }
            }
        }

        // Low confidence fallback
        if max_score < 0.5 {
            return (IntentState::Unknown, max_score);
        }

        (best_intent, max_score)
    }
}
