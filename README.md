# PingMe

**PingMe** is an open-source, Rust-powered notification platform that lets developers receive real-time alerts from their apps, services, or smart contracts. Whether you're monitoring backend logic, infrastructure, or blockchain events, PingMe helps you stay in the loop via email, Telegram, or custom webhooks.

## 🚀 Features

- 🔔 Real-time event-driven notifications
- 📨 Multiple delivery channels (Email, Telegram, Webhook)
- 🧱 Built for performance and scalability with Rust
- 📡 Simple API to trigger notifications from any app
- 🛠️ Easily extensible — add your own integrations

## 💡 Use Cases

- Get notified when a backend process completes or fails
- Trigger alerts on smart contract events (e.g. using Starknet)
- Send system updates, warnings, or custom messages

## 🦀 Tech Stack

- **Rust** (safe & performant)
- **Axum** — Web framework for APIs
- **Tokio** — Async runtime for concurrency
- **sqlx** or **SurrealDB** — Flexible database layer (choose one)
- **Lettre** — For sending emails
- **Teloxide** — For Telegram bot integration
- **Serde** — For serialization
- **dotenv** — Environment configuration

## ⚙️ Environment Configuration

1. Copy `.env.example` to `.env` and fill in your secrets:
   ```sh
   cp .env.example .env
   # Edit .env to add your API keys and database URL
   ```
2. The application will load and validate required environment variables at startup. If any are missing, it will exit with an error message.

## 🛠️ Getting Started

1. Clone the repository:
   ```sh
   git clone https://github.com/yourusername/pingme.git
   cd pingme
   ```
2. Install Rust and Cargo if you haven't already.
3. Build the project:
   ```sh
   cargo build --release
   ```
4. Run the application:
   ```sh
   cargo run
   ```
5. Send a test notification:
   ```sh
   curl -X POST http://localhost:3000/notify -H "Content-Type: application/json" -d '{"message": "Hello, world!"}'
   ```
6. Check your email, Telegram, or webhook receiver for the notification.

