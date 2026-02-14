# Tournament Server

> A Rust-based server for managing single-elimination tournament brackets for image-based idea competitions.

## Overview

This server provides a backend infrastructure for running tournaments where ideas, represented through images, compete in a by match elimination format. Built with Rust and leveraging MongoDB for data persistence, the system offers secure authentication and efficient tournament management.

## 🚀 Features

- **JWT-based Authentication**: Secure user authentication system
- **Tournament Management**: Create and manage single-elimination tournaments
- **Image Handling**: Support for idea representation through images

## 🛠 Tech Stack

- **Language**: Rust
- **Web Framework**: Rocket.rs
- **Database**: MongoDB
- **Storage**: AWS S3
- **Authentication**: JWT (JSON Web Tokens)
- **Testing**: Built-in Rust testing framework

## 📋 Prerequisites

- Rust (latest stable version)
- MongoDB
- AWS CLI / S3 access
- Cargo (Rust package manager)

## ⚙️ Configuration

The server requires the following environment variables:

```env
# AWS S3 Configuration
AWS_REGION=AWS_region
AWS_ACCESS_KEY_ID=AWS_access_key_id
AWS_SECRET_ACCESS_KEY=AWS_secret_access_key
AWS_S3_BUCKET=S3_bucket_name

# JWT Configuration
JWT_SECRET=Secret_key_for_JWT_signing

# MongoDB Configuration
MONGODB_URI=MongoDB_connection_string
MONGODB_NAME=MongoDB_database_name
```

## 🚀 Getting Started

1. Clone the repository

```bash
git clone https://github.com/wokuApp/torvi
```

2. Install dependencies

```bash
cargo build
```

3. Run the server

```bash
cargo run
```

4. Run tests

```bash
cargo test
```

## 🔒 API Authentication

The API uses Bearer token authentication. Include the JWT token in the Authorization header:

Authorization: Bearer <your_token>

## 👥 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## 👤 Author

**Diego Orrego** - CTO at Woku

- Email: diego@woku.app
- GitHub: [@diorrego](https://github.com/diorrego)

## 🏢 Company

[woku](https://woku.app)

---

Built with ❤️ by Diego Orrego
