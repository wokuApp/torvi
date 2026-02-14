use rocket::fairing::AdHoc;

pub struct S3Config {
    pub region: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub bucket: String,
}

impl S3Config {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            region: std::env::var("AWS_REGION")
                .map_err(|_| "Missing AWS_REGION environment variable")?,
            access_key_id: std::env::var("AWS_ACCESS_KEY_ID")
                .map_err(|_| "Missing AWS_ACCESS_KEY_ID environment variable")?,
            secret_access_key: std::env::var("AWS_SECRET_ACCESS_KEY")
                .map_err(|_| "Missing AWS_SECRET_ACCESS_KEY environment variable")?,
            bucket: std::env::var("AWS_S3_BUCKET")
                .map_err(|_| "Missing AWS_S3_BUCKET environment variable")?,
        })
    }
}

pub fn init() -> AdHoc {
    AdHoc::on_ignite("S3 Configuration", |rocket| async {
        match S3Config::from_env() {
            Ok(config) => rocket.manage(config),
            Err(e) => {
                panic!("Failed to initialize S3 configuration: {}", e);
            }
        }
    })
}
