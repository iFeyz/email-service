#[derive(Debug, Clone)]
pub struct SmtpConfig {
    pub server: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub sender_email: String,
}

impl Default for SmtpConfig {
    fn default() -> Self {
        Self {
            server: "YOUR_SMTP_SERVER".to_string(),
            port: 465,
            username: "YOUR_SMTP_USERNAME".to_string(),
            password: "YOUR_SMTP_PASSWORD".to_string(),
            sender_email: "YOUR_SENDER_EMAIL".to_string(),
        }
    }
}