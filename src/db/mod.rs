use sqlx::postgres::{PgPool, PgPoolOptions};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::email_service::models::EmailRequest;

pub struct Database {
    pool: PgPool,
}

#[derive(sqlx::FromRow)]
pub struct Email {
    pub id: Uuid,
    pub to_address: String,
    pub subject: String,
    pub content: String,
    pub sent_at: DateTime<Utc>,
}

impl Database {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/email_service".to_string());

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn init(&self) -> Result<(), sqlx::Error> {
        // Créer l'extension uuid-ossp
        sqlx::query!(
            r#"CREATE EXTENSION IF NOT EXISTS "uuid-ossp";"#
        )
        .execute(&self.pool)
        .await?;

        // Créer la table emails
        sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS emails (
                id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
                to_address TEXT NOT NULL,
                subject TEXT NOT NULL,
                content TEXT NOT NULL,
                sent_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
            )
            "#
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn save_email(&self, request: &EmailRequest) -> Result<Uuid, sqlx::Error> {
        let id = sqlx::query_scalar!(
            r#"
            INSERT INTO emails (to_address, subject, content)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            request.to,
            request.subject,
            request.content
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(id)
    }

    pub async fn get_email(&self, id: Uuid) -> Result<Option<Email>, sqlx::Error> {
        let email = sqlx::query_as!(
            Email,
            r#"
            SELECT * FROM emails WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(email)
    }

    pub async fn list_emails(&self) -> Result<Vec<Email>, sqlx::Error> {
        let emails = sqlx::query_as!(
            Email,
            r#"
            SELECT * FROM emails ORDER BY sent_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(emails)
    }
}
