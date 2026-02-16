mod monitor;
mod intent;
mod optimizer;

use monitor::SystemMonitor;
use intent::IntentEngine;
use optimizer::PolicyEngine;
use std::{thread, time::Duration};

#[tokio::main]
async fn main() {
    println!("--- Zenith Background Service (v0.1) ---");
    println!("[Daemon] Initializing System Monitor...");
    let mut monitor = SystemMonitor::new();
    let intent_engine = IntentEngine::new();

    println!("[Daemon] Starting Monitoring Loop (Interval: 2s)...");

    loop {
        monitor.refresh();
        
        let cpu = monitor.get_cpu_usage();
        let (top_process, proc_cpu, top_pid) = monitor::SystemMonitor::get_top_process_info(&monitor); // We need to update monitor to return PID
        let intent = intent_engine.classify(&top_process);

        println!("[Monitor] Global CPU: {:.1}% | Top App: {} (PID: {}) ({:.1}%) -> Intent: {:?}", 
            cpu, top_process, top_pid, proc_cpu, intent);

        // Apply Optimization Policy
        PolicyEngine::apply(&intent, top_pid);

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
