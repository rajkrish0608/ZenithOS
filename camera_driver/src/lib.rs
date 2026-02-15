#![no_std]

#[derive(Debug, Clone, Copy)]
pub struct AttentionState {
    pub focus_score: f32, // 0.0 to 1.0
    pub is_looking_away: bool,
}

pub struct CameraDriver;

impl CameraDriver {
    /// Simulates analyzing the camera feed for head orientation.
    pub fn get_attention_state(&self) -> AttentionState {
        // Mock: Simulating a user who is focused.
        AttentionState {
            focus_score: 0.98,
            is_looking_away: false,
        }
    }
}
