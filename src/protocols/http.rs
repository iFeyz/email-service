use async_trait::async_trait;
use axum::{
    routing::{post, get},
    Router,
    Json,
    extract::State,
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
};
use lettre::{Transport, SmtpTransport};
use lettre::transport::smtp::Error as SmtpError;
use crate::email_service::{EmailService, EmailRequest};
use super::ProtocolAdapter;
use serde_json::json;
use tower_http::trace::{TraceLayer, DefaultMakeSpan};
use crate::db::Database;

pub struct HttpAdapter<T: Transport<Error = SmtpError>> {
    email_service: EmailService<T>,
    addr: std::net::SocketAddr,
}

impl HttpAdapter<SmtpTransport> {
    pub fn new(email_service: EmailService<SmtpTransport>, addr: std::net::SocketAddr) -> Self {
        Self {
            email_service,
            addr,
        }
    }

    async fn handle_test() -> impl IntoResponse {
        println!("Test endpoint called");
        "Server is running!"
    }

    async fn handle_email(
        headers: HeaderMap,
        State(email_service): State<EmailService<SmtpTransport>>,
        Json(request): Json<EmailRequest>,
    ) -> impl IntoResponse {
        if let Some(content_type) = headers.get("content-type") {
            println!("Received Content-Type: {:?}", content_type);
            if content_type != "application/json" {
                return (
                    StatusCode::UNSUPPORTED_MEDIA_TYPE,
                    Json(json!({
                        "error": "Content-Type must be application/json"
                    }))
                ).into_response();
            }
        }

        println!("Received email request to: {}", request.to);
        println!("Full request: {:?}", request);

        // Connexion et initialisation de la base de données
        let db = match Database::new().await {
            Ok(db) => {
                // Initialisation de la base de données
                if let Err(e) = db.init().await {
                    eprintln!("Failed to initialize database: {}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({
                            "error": format!("Failed to initialize database: {}", e)
                        }))
                    ).into_response();
                }
                db
            },
            Err(e) => {
                eprintln!("Failed to connect to database: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": format!("Failed to connect to database: {}", e)
                    }))
                ).into_response();
            }
        };

        // Envoi de l'email
        match email_service.send_email(request.clone()).await {
            Ok(response) => {
                // Sauvegarde dans la base de données
                match db.save_email(&request).await {
                    Ok(email_id) => {
                        println!("Email saved to database with id: {}", email_id);
                        println!("Email sent successfully with id: {}", response.message_id);
                        (StatusCode::OK, Json(response)).into_response()
                    },
                    Err(e) => {
                        eprintln!("Failed to save email: {}", e);
                        // On continue même si la sauvegarde échoue, mais on log l'erreur
                        (StatusCode::OK, Json(response)).into_response()
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to send email: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": format!("Failed to send email: {}", e)
                    }))
                ).into_response()
            }
        }
    }
}

#[async_trait]
impl ProtocolAdapter for HttpAdapter<SmtpTransport> {
    async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Configuring HTTP routes...");
        
        let app = Router::new()
            .route("/", get(|| async { "Server is running!" }))
            .route("/email", post(Self::handle_email))
            .nest("/api", Router::new()
                .route("/email", post(Self::handle_email)))
            .with_state(self.email_service.clone())
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::default().include_headers(true))
            );

        println!("Starting HTTP server on {}", self.addr);
        println!("Available routes:");
        println!("  GET  /");
        println!("  POST /email");
        println!("  POST /api/email");

        axum::serve(
            tokio::net::TcpListener::bind(&self.addr).await?,
            app.into_make_service(),
        )
        .await?;

        Ok(())
    }

    async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
