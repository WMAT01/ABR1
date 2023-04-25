
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use uuid::Uuid;

// ProjectTask
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectTask {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub keywords: HashSet<String>,
    pub pt_checks: Vec<PTCheck>,
}

// PTCheck
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PTCheck {
    pub id: Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub text_diff_status: String,
    pub visual_diff_status: String,
    pub web_diff_status: String,
    pub compressed_response: Vec<u8>,
}

// User
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DatabaseType {
    SurrealDB,
    SQLite3,
}

pub struct User {
    // ...
    pub database_type: DatabaseType,
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub signup_timestamp: chrono::DateTime<chrono::Utc>,
    pub active_keywords: HashSet<String>,
    pub database_path: String,
    pub database_uuid: Uuid,
    pub timer_settings_on: bool,
    pub timer_settings_start: chrono::DateTime<chrono::Utc>,
    pub timer_settings_duration_hours: f32,
    pub timer_settings_checks: i32,
    pub billing_status: String,
}
