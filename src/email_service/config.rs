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
            server: "ssl0.ovh.net".to_string(),
            port: 465,
            username: "sender@wayfe.store".to_string(),
            password: "wayfesender".to_string(),
            sender_email: "sender@wayfe.store".to_string(),
        }
    }
}