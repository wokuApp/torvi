use rocket::fairing::AdHoc;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

pub fn build_cors_options() -> CorsOptions {
    let origins_str =
        std::env::var("CORS_ALLOWED_ORIGINS").unwrap_or_else(|_| "http://localhost:3000".into());

    let origins: Vec<String> = origins_str
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let origin_refs: Vec<&str> = origins.iter().map(|s| s.as_str()).collect();
    let allowed_origins = AllowedOrigins::some_exact(&origin_refs);

    let allowed_methods: rocket_cors::AllowedMethods = vec![
        Method::Get,
        Method::Post,
        Method::Put,
        Method::Patch,
        Method::Delete,
        Method::Options,
    ]
    .into_iter()
    .map(From::from)
    .collect();

    CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Content-Type",
            "Origin",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
}

pub fn init() -> AdHoc {
    AdHoc::on_ignite("CORS Configuration", |rocket| async {
        let cors = build_cors_options()
            .to_cors()
            .expect("Failed to build CORS configuration");
        rocket.attach(cors)
    })
}
