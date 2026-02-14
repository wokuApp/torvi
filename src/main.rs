#[macro_use]
extern crate rocket;

use dotenv::dotenv;

mod config;
pub mod common;
pub mod error;
mod modules;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(config::database::init())
        .attach(config::s3::init())
        .attach(config::jwt::init())
        .attach(config::indices::init())
        .attach(config::services::init())
        .mount("/api/opponents", modules::opponents::routes())
        .mount("/api/tournaments", modules::tournaments::routes())
        .mount("/api/users", modules::users::routes())
        .mount("/api/images", modules::images::routes())
        .mount("/api/auth", modules::auth::routes())
}
