use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};
use chrono::prelude::*;

pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        // Initialize Tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS events (
                id INTEGER PRIMARY KEY,
                timestamp TEXT NOT NULL,
                intent TEXT NOT NULL,
                app_name TEXT NOT NULL,
                score INTEGER NOT NULL,
                mode TEXT NOT NULL
            )",
            [],
        )?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn log_event(&self, intent: &str, app_name: &str, score: i32, mode: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let timestamp = Local::now().to_rfc3339();
        
        // Privacy Guard: Ensure app_name is not a window title or path
        let safe_app_name = app_name.split('/').last().unwrap_or(app_name); // simplistic binary name check
        
        conn.execute(
            "INSERT INTO events (timestamp, intent, app_name, score, mode) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![timestamp, intent, safe_app_name, score, mode],
        )?;
        Ok(())
    }

    // Future: Get history for charts
    #[allow(dead_code)]
    pub fn get_recent_events(&self, limit: i32) -> Result<Vec<(String, i32)>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT timestamp, score FROM events ORDER BY id DESC LIMIT ?1")?;
        
        let events = stmt.query_map(params![limit], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(events)
    }
}
