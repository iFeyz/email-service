use tonic::{transport::Server, Request, Response, Status};
use crate::email_service::{EmailService, EmailRequest};
use super::ProtocolAdapter;
use async_trait::async_trait;
use lettre::Transport;
use lettre::transport::smtp::Error as SmtpError;

// Import du code généré par tonic
pub mod email {
    tonic::include_proto!("email");
}

use email::email_service_server::{EmailService as GrpcEmailService, EmailServiceServer};

#[derive(Clone)]
pub struct GrpcAdapter<T: Transport<Error = SmtpError> + Clone + Send + Sync + 'static> {
    email_service: EmailService<T>,
    addr: std::net::SocketAddr,
}

impl<T: Transport<Error = SmtpError> + Clone + Send + Sync + 'static> GrpcAdapter<T> {
    pub fn new(email_service: EmailService<T>, addr: std::net::SocketAddr) -> Self {
        Self {
            email_service,
            addr,
        }
    }
}

#[tonic::async_trait]
impl<T: Transport<Error = SmtpError> + Clone + Send + Sync + 'static> GrpcEmailService for GrpcAdapter<T> {
    async fn send_email(
        &self,
        request: Request<email::EmailRequest>,
    ) -> Result<Response<email::EmailResponse>, Status> {
        let proto_req = request.into_inner();
        
        let email_req = EmailRequest {
            to: proto_req.to,
            subject: proto_req.subject,
            content: proto_req.content,
            metadata: proto_req.metadata,
        };

        match self.email_service.send_email(email_req).await {
            Ok(response) => {
                let proto_resp = email::EmailResponse {
                    message_id: response.message_id,
                    status: response.status,
                    timestamp: response.timestamp.to_rfc3339(),
                };
                Ok(Response::new(proto_resp))
            }
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}

#[async_trait]
impl<T: Transport<Error = SmtpError> + Clone + Send + Sync + 'static> ProtocolAdapter for GrpcAdapter<T> {
    async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let svc = EmailServiceServer::new(self.clone());

        Server::builder()
            .add_service(svc)
            .serve(self.addr)
            .await?;

        Ok(())
    }

    async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}