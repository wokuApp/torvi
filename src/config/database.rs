use mongodb::{Client, Database};
use rocket::fairing::AdHoc;

#[derive(Clone, Debug)]
pub struct MongoDB {
    pub client: Client,
    pub db: Database,
}

impl MongoDB {
    pub async fn init() -> Result<Self, String> {
        let mongo_uri =
            std::env::var("MONGODB_URI").map_err(|_| "MONGODB_URI must be set".to_string())?;

        let client = Client::with_uri_str(&mongo_uri)
            .await
            .map_err(|e| format!("Failed to connect to MongoDB: {}", e))?;

        client
            .list_database_names()
            .await
            .map_err(|e| format!("Failed to verify connection: {}", e))?;

        let db_name = std::env::var("MONGODB_NAME").unwrap_or_else(|_| "torvi".to_string());

        Ok(MongoDB {
            client: client.clone(),
            db: client.database(&db_name),
        })
    }
}

pub fn init() -> AdHoc {
    AdHoc::on_ignite("MongoDB Database", |rocket| async {
        match MongoDB::init().await {
            Ok(mongodb) => rocket.manage(mongodb),
            Err(e) => {
                panic!("Failed to initialize MongoDB: {}", e);
            }
        }
    })
}
