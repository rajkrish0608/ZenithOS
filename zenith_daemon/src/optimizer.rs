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

use sysinfo::{System, SystemExt, ProcessExt, PidExt};

pub struct PolicyEngine;

impl PolicyEngine {
    pub fn apply(intent: &crate::intent::IntentState, top_pid: i32, sys: &System, mode: &str) {
        // 1. CHILL MODE override: Do nothing.
        if mode == "Chill" {
            return;
        }

        // 2. FOCUS MODE override: Always suppress distractions, regardless of intent.
        if mode == "Focus" {
             // Aggressively boost top app if it's not a distraction
             PriorityController::boost(top_pid);
             
             // Aggressively throttle distractions
             for (pid, process) in sys.processes() {
                let name = process.name();
                if name.contains("Slack") || name.contains("Discord") || name.contains("Teams") {
                    PriorityController::throttle(pid.as_u32() as i32);
                }
            }
            return;
        }

        // 3. STANDARD MODE (Default)
        match intent {
            crate::intent::IntentState::Coding | crate::intent::IntentState::Writing => {
                // 1. Boost the active app
                PriorityController::boost(top_pid);

                // 2. Distraction Suppression: Throttle Comm Apps
                for (pid, process) in sys.processes() {
                    let name = process.name();
                    if name.contains("Slack") || name.contains("Discord") || name.contains("Teams") {
                        // println!("[Policy] Throttling Distraction: {} (PID: {})", name, pid);
                        PriorityController::throttle(pid.as_u32() as i32);
                    }
                }
            },
            crate::intent::IntentState::Communication => {
                // If intent IS communication, ensure we un-throttle (reset nice to 0) 
                // For now, we just don't throttle.
            },
            _ => {}
        }
    }
}
