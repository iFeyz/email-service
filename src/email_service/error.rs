use thiserror::Error;
use lettre::transport::smtp::Error as SmtpError;

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("Failed to send email: {0}")]
    SmtpError(#[from] SmtpError),

    #[error("Invalid email address: {0}")]
    InvalidAddress(String),

    #[error("Template error: {0}")]
    TemplateError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}