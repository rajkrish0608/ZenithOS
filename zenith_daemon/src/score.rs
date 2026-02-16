use crate::intent::IntentState;

pub struct FocusCalculator {
    pub current_score: i32,
}

impl FocusCalculator {
    pub fn new() -> Self {
        Self { current_score: 50 }
    }

    pub fn update(&mut self, intent: &IntentState) -> i32 {
        match intent {
            IntentState::Coding | IntentState::Writing => {
                // Productivity Boost: +1 per tick (capped at 100)
                if self.current_score < 100 {
                    self.current_score += 1;
                }
            },
            IntentState::Browsing | IntentState::Communication => {
                // Distraction Penalty: -2 per tick (floor at 0)
                if self.current_score > 0 {
                    self.current_score -= 2;
                }
            },
            IntentState::Idle | IntentState::Unknown => {
                // Decay: -1 per tick
                if self.current_score > 0 {
                    self.current_score -= 1;
                }
            }
        }
        self.current_score
    }
}
