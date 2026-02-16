use std::process::Command;

pub struct PriorityController;

impl PriorityController {
    /// Boosts a process to high priority (nice -5) using `renice`.
    /// Requires sudo/root for negative nice values.
    pub fn boost(pid: i32) {
        let _ = Command::new("renice")
            .arg("-5") // Higher priority
            .arg("-p")
            .arg(pid.to_string())
            .output(); // Silently fail if no internal permission
            // In a real daemon, we'd log errors.
    }

    /// Throttles a process to low priority (nice +10).
    pub fn throttle(pid: i32) {
        let _ = Command::new("renice")
            .arg("10") // Lower priority
            .arg("-p")
            .arg(pid.to_string())
            .output();
    }
}

pub struct PolicyEngine;

impl PolicyEngine {
    pub fn apply(intent: &crate::intent::IntentState, top_pid: i32) {
        match intent {
            crate::intent::IntentState::Coding => {
                // Boost the coding app
                // println!("[Policy] Boosting PID {} for Coding Intent", top_pid);
                PriorityController::boost(top_pid);
            },
            crate::intent::IntentState::Writing => {
                 PriorityController::boost(top_pid);
            },
            crate::intent::IntentState::Communication => {
                // No boost, standard priority
            },
            _ => {}
        }
    }
}
