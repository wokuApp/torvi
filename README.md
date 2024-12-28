# Tournament Server

Un servidor basado en Rust para gestionar torneos de eliminación simple para competencias de ideas basadas en imágenes.

## 📖 Descripción General

Este servidor proporciona una infraestructura backend robusta para ejecutar torneos donde las ideas, representadas a través de imágenes, compiten en un formato de eliminación tipo bracket. Construido con Rust y MongoDB para la persistencia de datos, el sistema ofrece autenticación segura y gestión eficiente de torneos.

## ✨ Características

- **Autenticación JWT**: Sistema de autenticación seguro
- **Gestión de Torneos**: Creación y administración de torneos de eliminación simple
- **Manejo de Imágenes**: Soporte para representación de ideas mediante imágenes
- **Sistema de Brackets**: Generación y progresión automatizada de brackets
- **API RESTful**: Endpoints limpios y bien documentados

## 🛠️ Stack Tecnológico

- **Lenguaje**: Rust
- **Framework Web**: Rocket.rs
- **Base de Datos**: MongoDB
- **Autenticación**: JWT (JSON Web Tokens)
- **Testing**: Framework de testing integrado de Rust

## 📋 Requisitos Previos

- Rust (última versión estable)
- MongoDB
- Cargo (gestor de paquetes de Rust)

## ⚙️ Configuración

El servidor requiere las siguientes variables de entorno:

```console
MONGODB_URI
JWT_SECRET=your_jwt_secret

## 🚀 Getting Started

1. Clone the repository

bash
git clone [repository-url]

2. Install dependencies

bash
cargo build

3. Run the server

bash
cargo run

4. Run tests

bash
cargo test

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
```
