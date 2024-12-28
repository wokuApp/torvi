# Tournament Server

> A Rust-based server for managing single-elimination tournament brackets for image-based idea competitions.

## Overview

This server provides a robust backend infrastructure for running tournaments where ideas, represented through images, compete in a bracket-style elimination format. Built with Rust and leveraging MongoDB for data persistence, the system offers secure authentication and efficient tournament management.

## 🚀 Features

- **JWT-based Authentication**: Secure user authentication system
- **Tournament Management**: Create and manage single-elimination tournaments
- **Image Handling**: Support for idea representation through images
- **Bracket System**: Automated bracket generation and progression
- **RESTful API**: Clean and well-documented API endpoints

## 🛠 Tech Stack

- **Language**: Rust
- **Web Framework**: Rocket.rs
- **Database**: MongoDB
- **Authentication**: JWT (JSON Web Tokens)
- **Testing**: Built-in Rust testing framework

## 📋 Prerequisites

- Rust (latest stable version)
- MongoDB
- Cargo (Rust package manager)

## ⚙️ Configuration

The server requires the following environment variables:

env
MONGODB_URI=your_mongodb_connection_string
JWT_SECRET=your_jwt_secret

## 🚀 Getting Started

1. Clone the repository

```bash
git clone [repository-url]
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

This project is licensed under the MIT License - see the LICENSE file for details.

## 👤 Author

**Diego Orrego** - CTO at Woku

- Email: diego@woku.app
- GitHub: [@diorrego](https://github.com/diorrego)

## 🏢 Company

[Woku](https://woku.app)

---

Built with ❤️ by Diego Orrego
