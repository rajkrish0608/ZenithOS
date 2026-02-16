mod monitor;
mod intent;

use monitor::SystemMonitor;
use intent::IntentEngine;
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
        let (top_process, proc_cpu) = monitor.get_top_process();
        let intent = intent_engine.classify(&top_process);

        println!("[Monitor] Global CPU: {:.1}% | Top App: {} ({:.1}%) -> Intent: {:?}", 
            cpu, top_process, proc_cpu, intent);

        // In real service, we would adjust priority here based on intent
        if intent == intent::IntentState::Coding {
            // Placeholder: "Boost Mode Active"
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
