use serde::{Deserialize , Serialize};
use std::collections::HashMap;
use chrono::{DateTime , Utc};

#[derive(Debug , Deserialize)]
pub struct EmailRequest {
    pub from : String,
    pub to : String,
    pub subject : String,
    pub body : String,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug , Serialize)]
pub struct EmailResponse {
    pub status: String,
    pub message_id: String,
    pub timestamp: DateTime<Utc>,
}