use rocket::fairing::AdHoc;

pub struct JwtConfig {
    pub secret: String,
}

impl JwtConfig {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            secret: std::env::var("JWT_SECRET")
                .map_err(|_| "Missing JWT_SECRET environment variable")?,
        })
    }
}

pub fn init() -> AdHoc {
    AdHoc::on_ignite("JWT Configuration", |rocket| async {
        match JwtConfig::from_env() {
            Ok(config) => rocket.manage(config),
            Err(e) => {
                panic!("Failed to initialize JWT configuration: {}", e);
            }
        }
    })
}
