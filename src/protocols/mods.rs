use async_trait::async_trait;
use serde::{Deserialize , Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailRequest {
    pub recipient : String,
    pub subject : String,
    pub content : String,
    pub metadata : std::collections::HashMap<String, String>
}

#[derive(Debug , Clone , Serialize , Deserialize)]
pub struct EmailResponse {
    pub email_id : String,
    pub status : String,
}

#[async_trait]
pub trait ProtocolAdapter {
    async fn start(&self) -> Result<() , Box<dyn std::error::Error>>;
    async fn stop(&self) -> Result<() , Box<dyn std::error::Error>>;
}