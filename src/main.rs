mod protocols;
mod email_service;
mod db;
use crate::db::Database;
use protocols::{ProtocolAdapter, http::HttpAdapter, websocket::WebSocketAdapter, grpc::GrpcAdapter};
use email_service::{EmailService, config::SmtpConfig};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to database...");
    let db = Database::new().await?;
    
    println!("Initializing database...");
    db.init().await?;
    println!("Database initialized successfully!");

    println!("Configuring email service...");
    let config = SmtpConfig::default();
    let email_service = EmailService::with_config(config)?;

    // Configuration des adresses pour chaque service
    let http_addr = SocketAddr::from(([0, 0, 0, 0], 3030));
    let ws_addr = SocketAddr::from(([0, 0, 0, 0], 3031));
    let grpc_addr = SocketAddr::from(([0, 0, 0, 0], 3032));

    // Création des adaptateurs
    let http_adapter = HttpAdapter::new(email_service.clone(), http_addr);
    let websocket_adapter = WebSocketAdapter::new(email_service.clone(), ws_addr);
    let grpc_adapter = GrpcAdapter::new(email_service, grpc_addr);

    println!("Starting servers:");
    println!("  HTTP server on http://{}", http_addr);
    println!("  WebSocket server on ws://{}", ws_addr);
    println!("  gRPC server on {}", grpc_addr);

    // Démarrage de tous les services en parallèle
    tokio::join!(
        async {
            if let Err(e) = http_adapter.start().await {
                eprintln!("HTTP server error: {}", e);
            }
        },
        async {
            if let Err(e) = websocket_adapter.start().await {
                eprintln!("WebSocket server error: {}", e);
            }
        },
        async {
            if let Err(e) = grpc_adapter.start().await {
                eprintln!("gRPC server error: {}", e);
            }
        }
    );

    Ok(())
}

