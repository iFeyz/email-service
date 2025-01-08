# Multi-Protocol Email Service

A service for sending emails through multiple communication protocols (HTTP, WebSocket, gRPC) with PostgreSQL persistence.

## 🚀 Features

- ✉️ Email sending via SMTP
- 🌐 Multiple interfaces:
  - REST API (HTTP)
  - WebSocket
  - gRPC
- 📝 Data persistence with PostgreSQL
- 🐳 Docker containerization
- 🔄 Asynchronous request support

## 🛠️ Prerequisites

- Docker
- Docker Compose
- Rust (for development)
- PostgreSQL (for local development)

## 🔧 Configuration

### Environment Variables

Configure the following variables in `docker-compose.yml`:

```yaml
environment:
  - SMTP_SERVER=smtp.example.com
  - SMTP_PORT=587
  - SMTP_USERNAME=your_username
  - SMTP_PASSWORD=your_password
  - SENDER_EMAIL=sender@example.com
```

### Ports

- HTTP: 3030
- WebSocket: 3031
- gRPC: 3032
- PostgreSQL: 5432

## 📦 Installation

1. Clone the repository:
```bash
git clone [REPO_URL]
cd email-sender
```

2. Build and start the containers:
```bash
docker-compose up --build
```

## 🚀 Usage

### REST API (HTTP)

1. Server test:
```bash
curl http://localhost:3030/
```

2. Send an email:
```bash
curl -X POST http://localhost:3030/email \
  -H "Content-Type: application/json" \
  -d '{
    "to": "recipient@example.com",
    "subject": "Test",
    "content": "Message content"
  }'
```

### WebSocket

Connect to WebSocket server:
```javascript
const ws = new WebSocket('ws://localhost:3031');

ws.onopen = () => {
  ws.send(JSON.stringify({
    to: "recipient@example.com",
    subject: "WebSocket Test",
    content: "Message via WebSocket"
  }));
};

ws.onmessage = (event) => {
  console.log('Response:', event.data);
};
```

### gRPC

Use the proto file provided in `proto/email.proto` to generate gRPC clients.

## 🐳 Useful Docker Commands

### Container Management

```bash
# Start services
docker-compose up -d

# Stop services
docker-compose down

# View logs
docker-compose logs -f

# Restart specific service
docker-compose restart [service]
```

### Debugging

```bash
# Access container shell
docker-compose exec app sh

# Access database
docker-compose exec db psql -U postgres -d email_service
```

## 📊 Database Structure

`emails` table:
```sql
CREATE TABLE emails (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    to_address TEXT NOT NULL,
    subject TEXT NOT NULL,
    content TEXT NOT NULL,
    sent_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Tests with logging
RUST_LOG=debug cargo test
```

## 📝 Logging

Access logs via:
```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f app
docker-compose logs -f db
```

## 🔒 Security

- Use environment variables for sensitive information
- Configure PostgreSQL permissions properly
- Use HTTPS in production
- Limit access to unnecessary ports

## 🔄 Development

1. Code modifications:
```bash
# Stop containers
docker-compose down

# Rebuild and restart
docker-compose up --build
```

2. Database updates:
```bash
# Access database
docker-compose exec db psql -U postgres -d email_service
```

## 📚 API Documentation

### HTTP Endpoints

- `GET /`: Server availability test
- `POST /email`: Send an email
- `POST /api/email`: Alternative endpoint for sending email

### Request Format

```json
{
  "to": "recipient@example.com",
  "subject": "Subject",
  "content": "Content",
  "metadata": {} // Optional
}
```

### Response Format

Success:
```json
{
  "message_id": "unique-message-id",
  "status": "sent",
  "timestamp": "2024-01-08T15:34:56.056975Z"
}
```

Error:
```json
{
  "error": "Error description"
}
```

## 🔍 Monitoring

The service provides several monitoring endpoints:
- Health check: `GET /health`
- Metrics: `GET /metrics`
- Status: `GET /status`

## 🤝 What Next?

Here are some potential improvements and features you might want to explore:

### Feature Enhancements
- 📧 Email Templates Support
  - HTML templates
  - Markdown support
  - Template variables
- 🔄 Queue System
  - Message queuing with Redis/RabbitMQ
  - Retry mechanisms
  - Rate limiting
- 📊 Advanced Monitoring
  - Prometheus metrics
  - Grafana dashboards
  - Alert system

### Infrastructure
- ☁️ Cloud Deployment
  - AWS deployment guide
  - Azure setup
  - Google Cloud configuration
- 🔄 CI/CD Pipeline
  - GitHub Actions
  - Automated testing
  - Deployment automation

### Documentation
- 📚 API Documentation
  - OpenAPI/Swagger specs
  - Postman collection
  - Integration examples
- 🧪 Testing Guide
  - Integration tests
  - Load testing scenarios
  - Benchmark results

Feel free to contribute to any of these areas or suggest new improvements!

## 🤝 Contributing

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

MIT License - see the [LICENSE.md](LICENSE.md) file for details

## 📞 Support

For support, please:
1. Check the existing issues
2. Create a new issue with a detailed description
3. Include relevant logs and configuration

## 🙏 Acknowledgments

- Built with Rust and love
- Powered by Axum, Tokio, and SQLx
- Containerized with Docker 