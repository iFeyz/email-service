use lettre::{Transport, Message, address::Envelope};
use lettre::transport::smtp::Error as SmtpError;
use lettre::transport::smtp::response::{Response, Code};
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};
use super::SmtpConfig;

#[derive(Clone)]
pub struct MockSmtpTransport {
    pub sent_emails: Arc<Mutex<Vec<Message>>>,
    sender: mpsc::UnboundedSender<Message>,
}

impl MockSmtpTransport {
    pub fn new() -> Self {
        let (sender, mut receiver) = mpsc::unbounded_channel();
        let sent_emails = Arc::new(Mutex::new(Vec::new()));
        let emails_clone = sent_emails.clone();

        tokio::spawn(async move {
            while let Some(email) = receiver.recv().await {
                emails_clone.lock().await.push(email);
            }
        });

        Self {
            sent_emails,
            sender,
        }
    }

    pub async fn get_sent_emails(&self) -> Vec<Message> {
        self.sent_emails.lock().await.clone()
    }
}

impl Transport for MockSmtpTransport {
    type Error = SmtpError;
    type Ok = ();

    fn send(&self, email: &Message) -> Result<Self::Ok, Self::Error> {
        match self.sender.send(email.clone()) {
            Ok(_) => Ok(()),
            Err(e) => {
                let response = Response::new(
                    Code::new(4, 2, 1),
                    vec![format!("Failed to send email in mock: {}", e)]
                );
                Err(SmtpError::Permanent(response))
            }
        }
    }

    fn send_raw(&self, _envelope: &Envelope, _raw: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub fn create_test_config() -> SmtpConfig {
    SmtpConfig {
        server: "test.smtp.server".to_string(),
        port: 587,
        username: "test@example.com".to_string(),
        password: "test_password".to_string(),
        sender_email: "sender@test.com".to_string(),
    }
}