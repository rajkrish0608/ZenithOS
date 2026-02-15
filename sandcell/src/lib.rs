/// SandCell: A 2026-native isolation engine for Zenith OS.
/// Provides fine-grained, compiler-enforced sandboxes.

#[derive(Debug)]
pub enum SandboxError {
    AccessViolation(String),
    MemorySanitizationFailed,
    IsolationBreach,
}

pub trait IsolatedComponent {
    fn run(&self) -> Result<(), SandboxError>;
}

pub struct SandCell {
    pub name: String,
    pub permissions: Vec<String>,
}

impl SandCell {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            permissions: Vec::new(),
        }
    }

    pub fn grant_permission(&mut self, perm: &str) {
        self.permissions.push(perm.to_string());
    }

    pub fn enforce_isolation<F, T>(&self, action: F) -> Result<T, SandboxError>
    where
        F: FnOnce() -> T,
    {
        println!("[SandCell] Enforcing isolation for sandbox: {}", self.name);
        // Mocking a check: if an action is performed without explicit capability (simplified here)
        Ok(action())
    }

    pub fn terminate_on_violation(&self, reason: &str) {
        eprintln!("[SandCell] CRITICAL: Isolation violation in {}: {}. Terminating process.", self.name, reason);
        std::process::exit(1);
    }
}
