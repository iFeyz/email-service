use async_trait::async_trait;

pub mod grpc;
pub mod websocket;
pub mod http;

#[async_trait]
pub trait ProtocolAdapter {
    async fn start(&self) -> Result<(), Box<dyn std::error::Error>>;
    async fn stop(&self) -> Result<(), Box<dyn std::error::Error>>;
} 