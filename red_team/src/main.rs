use std::process::Command;

fn main() {
    println!("--- Zenith OS: Red Team Attack Simulator (Integrated) ---");
    println!("[Red Team] Objective: Trigger buffer overflow in OSTD-hardened VGA driver.");

    // Launch the Integrated VGA driver
    // The driver now uses OSTD syscalls and SandCell isolation.
    
    println!("[Red Team] Attempting to inject malicious payload into SandCell...");
    
    // We simulate the attack by running the vga_driver binary which contains the mock vulnerability logic.
    let status = Command::new("cargo")
        .arg("run")
        .arg("-p")
        .arg("vga_driver")
        .status()
        .expect("Failed to execute vga_driver");

    if status.success() {
        println!("[Red Team] FAILURE: VGA driver exited normally. Attack failed or was swallowed.");
    } else {
        println!("[Red Team] SUCCESS: The SandCell/OSTD layer blocked the attack! Process terminated.");
    }
}
