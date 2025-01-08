use lettre::{Message, Transport};
use lettre::message::header::ContentType;
use lettre::transport::smtp::Error as SmtpError;
use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;
use lettre::message::Mailbox;
use crate::email_service::error::EmailError;
use crate::email_service::models::{EmailRequest, EmailResponse};
use crate::email_service::config::SmtpConfig;
use chrono::Utc;
use uuid::Uuid;

#[derive(Clone)]
pub struct EmailService<T: Transport<Error = SmtpError>> {
    transport: T,
    sender_email: String,
}

impl EmailService<SmtpTransport> {
    pub fn with_config(config: SmtpConfig) -> Result<Self, EmailError> {
        let creds = Credentials::new(
            config.username.clone(),
            config.password.clone(),
        );

        let transport = SmtpTransport::relay(&config.server)
            .map_err(|e| EmailError::ConfigError(e.to_string()))?
            .port(config.port)
            .credentials(creds)
            .build();

        Ok(Self { 
            transport,
            sender_email: config.sender_email,
        })
    }
}

impl<T: Transport<Error = SmtpError>> EmailService<T> {
    pub async fn send_email(&self, request: EmailRequest) -> Result<EmailResponse, EmailError> {
        let email = Message::builder()
            .from(self.sender_email.parse::<Mailbox>()
                .map_err(|e| EmailError::InvalidAddress(e.to_string()))?)
            .to(request.to.parse::<Mailbox>()
                .map_err(|e| EmailError::InvalidAddress(e.to_string()))?)
            .subject(request.subject)
            .header(ContentType::TEXT_PLAIN)
            .body(request.content.clone())
            .map_err(|e| EmailError::ConfigError(e.to_string()))?;

        self.transport.send(&email)
            .map_err(|e| EmailError::SmtpError(e))?;

        Ok(EmailResponse {
            message_id: Uuid::new_v4().to_string(),
            status: "sent".to_string(),
            timestamp: Utc::now(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::email_service::test_utils::MockSmtpTransport;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_email_service_creation() {
        let service = EmailService::new();
        assert!(service.is_ok());
    }

    #[tokio::test]
    async fn test_send_email() {
        let mock_transport = MockSmtpTransport::new();
        let service = EmailService {
            smtp_client: mock_transport.clone(),
            sender_email: "sender@test.com".to_string(),
        };

        let request = EmailRequest {
            to: "test@example.com".to_string(),
            subject: "Test Email".to_string(),
            content: "This is a test email".to_string(),
            metadata: HashMap::new(),
        };

        let result = service.send_email(request).await;
        assert!(result.is_ok());

        let sent_emails = mock_transport.get_sent_emails().await;
        assert_eq!(sent_emails.len(), 1);
    }

    #[tokio::test]
    async fn test_invalid_email() {
        let mock_transport = MockSmtpTransport::new();
        let service = EmailService {
            smtp_client: mock_transport,
            sender_email: "sender@test.com".to_string(),
        };

        let request = EmailRequest {
            to: "invalid-email".to_string(),
            subject: "Test".to_string(),
            content: "Test content".to_string(),
            metadata: HashMap::new(),
        };

        let result = service.send_email(request).await;
        assert!(matches!(result, Err(EmailError::AddressError(_))));
    }
}