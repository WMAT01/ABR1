// src/database.rs
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    email: String,
    version: u32,
    sign_up_timestamp: DateTime<Utc>,
    active_keywords: Vec<String>,
    database_path: String,
    database_uuid: Uuid,
    timer_settings_on: bool,
    timer_settings_start: DateTime<Utc>,
    timer_settings_duration_hours: f64,
    timer_settings_checks: u32,
    billing_status: String,
}

pub fn migrate_user(user: &mut User) -> Result<(), DatabaseError> {
    let current_version = 1; // Update this as needed

    if user.version == current_version {
        return Ok(());
    } else if user.version > current_version {
        return Err(DatabaseError::VersioningError);
    }

    // Migrate user data here
    // ...
    
    user.version = current_version;
    Ok(())
}


#[derive(Debug, PartialEq, Eq)]
pub enum DatabaseError {
    MigrationError,
    VersioningError,
    // Add other errors here as needed
}


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
