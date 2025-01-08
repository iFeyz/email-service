use async_trait::async_trait;
use tokio::net::{TcpListener, TcpStream};
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::accept_async;
use lettre::Transport;
use lettre::transport::smtp::Error as SmtpError;
use crate::email_service::{EmailService, EmailRequest};
use super::ProtocolAdapter;

#[derive(Clone)]
pub struct WebSocketAdapter<T: Transport<Error = SmtpError> + Clone + Send + Sync + 'static> {
    email_service: EmailService<T>,
    addr: std::net::SocketAddr,
}

impl<T: Transport<Error = SmtpError> + Clone + Send + Sync + 'static> WebSocketAdapter<T> {
    pub fn new(email_service: EmailService<T>, addr: std::net::SocketAddr) -> Self {
        Self {
            email_service,
            addr,
        }
    }

    async fn handle_connection(stream: TcpStream, email_service: EmailService<T>) {
        let ws_stream = accept_async(stream).await.expect("Failed to accept");
        let (mut write, mut read) = ws_stream.split();

        while let Some(msg) = read.next().await {
            if let Ok(msg) = msg {
                if let Ok(request) = serde_json::from_str::<EmailRequest>(&msg.to_string()) {
                    match email_service.send_email(request).await {
                        Ok(response) => {
                            if let Ok(response_json) = serde_json::to_string(&response) {
                                let _ = write.send(response_json.into()).await;
                            }
                        }
                        Err(e) => {
                            let error_response = serde_json::json!({
                                "error": e.to_string()
                            });
                            let _ = write.send(error_response.to_string().into()).await;
                        }
                    }
                }
            }
        }
    }
}

#[async_trait]
impl<T: Transport<Error = SmtpError> + Clone + Send + Sync + 'static> ProtocolAdapter for WebSocketAdapter<T> {
    async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(self.addr).await?;
        let email_service = self.email_service.clone();

        tokio::spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                let email_service = email_service.clone();
                tokio::spawn(Self::handle_connection(stream, email_service));
            }
        });

        Ok(())
    }

    async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}