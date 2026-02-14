#[macro_use]
extern crate rocket;

use dotenvy::dotenv;

mod config;
pub mod common;
pub mod error;
mod modules;
mod spa;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    config::logging::init();

    rocket::build()
        .attach(config::request_id::init())
        .attach(config::security::init())
        .attach(config::cors::init())
        .attach(config::database::init())
        .attach(config::s3::init())
        .attach(config::jwt::init())
        .attach(config::indices::init())
        .attach(config::services::init())
        .mount("/health", modules::health::routes())
        .mount("/api/opponents", modules::opponents::routes())
        .mount("/api/tournaments", modules::tournaments::routes())
        .mount("/api/users", modules::users::routes())
        .mount("/api/images", modules::images::routes())
        .mount("/api/auth", modules::auth::routes())
        .mount("/ws", modules::websocket::routes())
        .mount("/", rocket::fs::FileServer::from("web/landing/out").rank(10))
        .mount("/", spa::routes())
}
