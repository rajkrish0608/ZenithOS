use zapi::ai;
use zapi::core;

fn main() {
    println!("--- Mission: Hello Zenith ---");
    println!("[HelloZenith] Initializing...");
    
    // Simulate some work
    core::sleep(500);

    // AI Query
    let focus = ai::get_focus_score();
    println!("[HelloZenith] My AI Focus Score is: {:.2}", focus);

    if focus > 0.8 {
        println!("[HelloZenith] High focus detected! Creating optimal environment.");
    } else {
        println!("[HelloZenith] Low focus. Running in background mode.");
    }

    let answer = ai::query_knowledge("What is the meaning of life?");
    println!("[HelloZenith] AI Knowledge Query Result: {}", answer);
}
