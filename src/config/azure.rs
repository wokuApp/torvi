use rocket::fairing::AdHoc;

pub struct AzureConfig {
    pub storage_account: String,
    pub access_key: String,
    pub container: String,
}

impl AzureConfig {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            storage_account: std::env::var("AZURE_STORAGE_ACCOUNT")
                .map_err(|_| "Missing AZURE_STORAGE_ACCOUNT environment variable")?,
            access_key: std::env::var("AZURE_STORAGE_KEY")
                .map_err(|_| "Missing AZURE_STORAGE_KEY environment variable")?,
            container: std::env::var("AZURE_STORAGE_CONTAINER")
                .map_err(|_| "Missing AZURE_STORAGE_CONTAINER environment variable")?,
        })
    }
}

pub fn init() -> AdHoc {
    AdHoc::on_ignite("Azure Configuration", |rocket| async {
        match AzureConfig::from_env() {
            Ok(config) => rocket.manage(config),
            Err(e) => {
                panic!("Failed to initialize Azure configuration: {}", e);
            }
        }
    })
}
