match user.database_type {
    DatabaseType::SurrealDB => {
        // Call SurrealDB functions from database.rs
    }
    DatabaseType::SQLite3 => {
        // Call SQLite3 functions from dbsqlite3.rs
    }
}
// src/database.rs
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectTask {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub keywords: Vec<String>,
    pub pt_checks: Vec<PTCheck>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PTCheck {
    pub id: i32,
    pub timestamp: SystemTime,
    pub compressed_response: Vec<u8>,
    pub status: Option<String>,
}

impl ProjectTask {
    pub fn new(id: i32, name: String, url: String, keywords: Vec<String>) -> Self {
        Self {
            id,
            name,
            url,
            keywords,
            pt_checks: Vec::new(),
        }
    }
}

impl PTCheck {
    pub fn new(id: i32, compressed_response: Vec<u8>) -> Self {
        Self {
            id,
            timestamp: SystemTime::now(),
            compressed_response,
            status: None,
        }
    }
}

pub fn create_database(email: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let db_name = format!("{}-{}", email, timestamp);
    // Create the database with the given name
    // ...
    db_name
}

pub fn open_latest_database(email: &str) -> String {
    // Find the latest database for the given email
    // ...
    // Load the data from the database
    // ...
    // Return the database name
    "latest_database_name".to_string()
}
