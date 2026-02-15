use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// HelixDB: A local-first vector store for OS audit trails.
/// Logs every IPC communication for real-time security analysis.

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub source: String,
    pub destination: String,
    pub action: String,
    pub security_score: f32, // 1.0 is safe, 0.0 is anomaly
    pub payload_hash: String,
}

pub struct HelixDB {
    logs: Vec<AuditEntry>,
}

impl HelixDB {
    pub fn new() -> Self {
        Self { logs: Vec::new() }
    }

    pub fn log_event(&mut self, source: &str, dest: &str, action: &str, score: f32) {
        let entry = AuditEntry {
            timestamp: Utc::now(),
            source: source.to_string(),
            destination: dest.to_string(),
            action: action.to_string(),
            security_score: score,
            payload_hash: "mock_hash".to_string(),
        };
        
        println!("[HelixDB] Audit: source={}, dest={}, action={}, score={:.2}", 
                 source, dest, action, score);
        
        self.logs.push(entry);
        
        if score < 0.5 {
            println!("[HelixDB] ALERT: Security anomaly detected in IPC event!");
        }
    }
}
