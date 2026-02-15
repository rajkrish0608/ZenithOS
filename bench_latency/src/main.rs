use std::time::Instant;
use scheduler::{Scheduler, ProcessType};

fn main() {
    println!("--- Zenith OS: Intelligence vs Latency Benchmark ---");
    println!("[Benchmark] Target: < 10ms Inference Overhead");

    let mut scheduler = Scheduler::new();
    scheduler.register_process(1, "VS Code", ProcessType::ForegroundMission);
    scheduler.register_process(2, "Social Media", ProcessType::BackgroundSlop);
    
    let iterations = 100;
    let mut total_duration = 0;

    println!("[Benchmark] Running {} scheduling cycles...", iterations);

    for i in 0..iterations {
        // Simulate rapid focus switching
        if i % 2 == 0 {
            scheduler.set_active_window("term_zenith");
        } else {
            scheduler.set_active_window("browser_social");
        }

        let start = Instant::now();
        scheduler.schedule();
        let duration = start.elapsed();
        
        total_duration += duration.as_micros();
    }

    let avg_latency = total_duration as f32 / iterations as f32 / 1000.0; // ms
    println!("\n[Result] Average Scheduling Latency: {:.4} ms", avg_latency);

    if avg_latency < 10.0 {
        println!("[PASS] Scheduler overhead is within limits.");
    } else {
        println!("[FAIL] Latency exceeded 10ms limit!");
        std::process::exit(1);
    }
}
