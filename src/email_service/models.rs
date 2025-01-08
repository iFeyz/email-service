use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Deserialize)]
pub struct EmailRequest {
    pub to: String,
    pub subject: String,
    pub content: String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmailResponse {
    pub status: String,
    pub message_id: String,
    pub timestamp: DateTime<Utc>,
} 