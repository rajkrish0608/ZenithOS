mod monitor;
mod intent;
mod optimizer;
mod server;
mod score;

use monitor::SystemMonitor;
use intent::IntentEngine;
use optimizer::PolicyEngine;
use server::{AppState, SharedState};
use score::FocusCalculator;
use std::{thread, time::Duration};
use std::sync::{Arc, RwLock};

#[tokio::main]
async fn main() {
    println!("--- Zenith Background Service (v0.1) ---");
    
    // 1. Initialize State
    let state = Arc::new(RwLock::new(AppState {
        cpu: 0.0,
        intent: "Initializing...".to_string(),
        app: "Scanning...".to_string(),
        pid: 0,
        score: 50,
        mode: "Standard".to_string(),
    }));

    // 2. Start Web Server (Spawn task)
    let server_state = state.clone();
    tokio::spawn(async move {
        server::start_server(server_state).await;
    });

    println!("[Daemon] Initializing System Monitor...");
    let mut monitor = SystemMonitor::new();
    let intent_engine = IntentEngine::new();
    let mut focus_calc = FocusCalculator::new();

    println!("[Daemon] Starting Monitoring Loop (Interval: 2s)...");

    loop {
        monitor.refresh();
        
        let cpu = monitor.get_cpu_usage();
        let (top_process, proc_cpu, top_pid) = monitor::SystemMonitor::get_top_process_info(&monitor);
        let (intent, confidence) = intent_engine.infer(&top_process);
        
        // Calculate Score
        let new_score = focus_calc.update(&intent);

        // Update Shared State for UI
        {
            let mut data = state.write().unwrap();
            data.cpu = cpu;
            data.app = top_process.clone();
            data.intent = format!("{:?} ({:.0}%)", intent, confidence * 100.0);
            data.pid = top_pid;
            data.score = new_score;
        }

        // Retrieve current mode safely
        let mode = {
            state.read().unwrap().mode.clone()
        };

        println!("[Monitor] Global CPU: {:.1}% | Top App: {} (PID: {}) ({:.1}%) -> Intent: {:?} (Conf: {:.2}) | Score: {} | Mode: {}", 
            cpu, top_process, top_pid, proc_cpu, intent, confidence, new_score, mode);

        // Apply Optimization Policy
        PolicyEngine::apply(&intent, top_pid, &monitor.sys, &mode);

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
