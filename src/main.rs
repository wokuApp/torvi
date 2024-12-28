use dotenv::dotenv;

#[macro_use]

mod config;
pub mod common;
mod modules;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(config::database::init())
        .attach(config::azure::init())
        .attach(config::jwt::init())
        .mount("/api/opponents", modules::opponents::routes())
        .mount("/api/tournaments", modules::tournaments::routes())
        .mount("/api/users", modules::users::routes())
        .mount("/api/images", modules::images::routes())
        .mount("/api/auth", modules::auth::routes())
}
