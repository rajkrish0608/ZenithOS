use std::collections::HashMap;

/// Intent-Aware Scheduler for Zenith OS.
/// Uses a lightweight inference model to prioritize user focus.

#[derive(Debug, Clone, PartialEq)]
pub enum ProcessType {
    ForegroundMission,
    BackgroundSlop,
    KernelCritical,
}

#[derive(Debug, Clone)]
pub struct Process {
    pub name: String,
    pub p_type: ProcessType,
    pub cpu_usage: f32, // 0.0 to 1.0
    pub memory_usage_mb: u32,
    pub max_memory_quota: u32,
}

impl Process {
    pub fn check_quota(&self) {
        if self.memory_usage_mb > self.max_memory_quota {
             println!("[Scheduler] ALERT: Process {} exceeded Memory Quota ({} > {} MB). Terminating...", 
                self.name, self.memory_usage_mb, self.max_memory_quota);
             // In real OS: kill signal
        }
    }
}

pub struct IntentModel;

impl IntentModel {
    /// Mock inference: Predicts user focus score (0.0 - 1.0)
    /// In a real system, this would query an NPU-accelerated model.
    pub fn predict_focus_score(active_window: &str) -> f32 {
        match active_window {
            "term_zenith" | "code_editor" => 0.95,
            "browser_social" => 0.3,
            _ => 0.1,
        }
    }
}

pub struct Scheduler {
    processes: HashMap<u64, Process>,
    active_window: String,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            processes: HashMap::new(),
            active_window: "term_zenith".to_string(),
        }
    }

    pub fn register_process(&mut self, id: u64, name: &str, p_type: ProcessType) {
        let quota = match p_type {
            ProcessType::ForegroundMission => 1024, // 1GB
            ProcessType::BackgroundSlop => 128,   // 128MB
            ProcessType::KernelCritical => 4096,
        };

        self.processes.insert(id, Process {
            id,
            name: name.to_string(),
            p_type,
            cpu_usage: 0.0,
            memory_usage_mb: 50, // Mock usage
            max_memory_quota: quota,
        });
    }

    pub fn set_active_window(&mut self, window: &str) {
        self.active_window = window.to_string();
    }

    pub fn schedule(&mut self) {
        let focus_score = IntentModel::predict_focus_score(&self.active_window);
        println!("[Scheduler] User Focus Score: {:.2} (Context: {})", focus_score, self.active_window);

        for process in self.processes.values_mut() {
            process.check_quota();
            match process.p_type {
                ProcessType::KernelCritical => {
                    process.cpu_usage = 1.0; // Always runs
                }
                ProcessType::ForegroundMission => {
                    // High focus grants max CPU to mission
                    if focus_score > 0.8 {
                        process.cpu_usage = 0.9;
                    } else {
                        process.cpu_usage = 0.5;
                    }
                }
                ProcessType::BackgroundSlop => {
                    // Aggressive throttling if user is focused
                    if focus_score > 0.8 {
                        process.cpu_usage = 0.05; // < 5% restriction
                        println!("[Scheduler] THROTTLING SLOP: {} limited to 5% CPU", process.name);
                    } else {
                        process.cpu_usage = 0.3;
                    }
                }
            }
        }
    }
}
