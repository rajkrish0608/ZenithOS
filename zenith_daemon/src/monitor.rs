use sysinfo::{ProcessExt, System, SystemExt, CpuExt};
use std::collections::HashMap;

pub struct SystemMonitor {
    sys: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
            sys: System::new_all(),
        }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();
    }

    pub fn get_cpu_usage(&self) -> f32 {
        self.sys.global_cpu_info().cpu_usage()
    }

    pub fn get_top_process(&self) -> (String, f32) {
        let mut top_pid = 0;
        let mut max_cpu = 0.0;
        let mut name = String::new();

        for (pid, process) in self.sys.processes() {
            if process.cpu_usage() > 0.1 {
                // Debug print
                // println!("PID: {} | Name: {} | CPU: {:.1}", pid, process.name(), process.cpu_usage());
            }
            if process.cpu_usage() > max_cpu {
                max_cpu = process.cpu_usage();
                name = process.name().to_string();
            }
        }
        (name, max_cpu)
    }
}
