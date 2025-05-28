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

## 🛠️ Getting Started

