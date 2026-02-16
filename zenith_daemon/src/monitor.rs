use sysinfo::{ProcessExt, System, SystemExt, CpuExt, PidExt};


pub struct SystemMonitor {
    pub sys: System,
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

    pub fn get_top_process_info(&self) -> (String, f32, i32) {
        let mut top_pid_out = 0;
        let mut max_cpu = 0.0;
        let mut name = String::new();

        for (pid, process) in self.sys.processes() {
            if process.cpu_usage() > max_cpu {
                max_cpu = process.cpu_usage();
                name = process.name().to_string();
                top_pid_out = pid.as_u32() as i32; // sysinfo Pid can be cast
            }
        }
        (name, max_cpu, top_pid_out)
    }
}
