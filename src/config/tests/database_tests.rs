use crate::config::database::MongoDB;
use mongodb::Client;
use std::env;

fn setup() {
    env::remove_var("MONGODB_URI");
    env::remove_var("MONGODB_NAME");
}

#[tokio::test]
async fn test_mongodb_init_success() {
    // Arrange
    setup();
    env::set_var("MONGODB_URI", "mongodb://localhost:27017");
    env::set_var("MONGODB_NAME", "test_db");

    // Act
    let result = MongoDB::init().await;

    // Assert
    assert!(result.is_ok());
    let mongodb = result.unwrap();
    assert_eq!(mongodb.db.name(), "test_db");
}

#[tokio::test]
async fn test_mongodb_init_default_db_name() {
    // Arrange
    setup();
    env::set_var("MONGODB_URI", "mongodb://localhost:27017");
    // No MONGODB_NAME set

    // Act
    let result = MongoDB::init().await;

    // Assert
    assert!(result.is_ok());
    let mongodb = result.unwrap();
    assert_eq!(mongodb.db.name(), "project"); // Default value
}

#[tokio::test]
async fn test_mongodb_init_missing_uri() {
    // Arrange
    setup();
    // No MONGODB_URI set

    // Act
    let result = MongoDB::init().await;

    // Assert
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "MONGODB_URI must be set");
}

#[tokio::test]
async fn test_mongodb_init_invalid_uri() {
    // Arrange
    setup();
    env::set_var("MONGODB_URI", "invalid-uri");

    // Act
    let result = MongoDB::init().await;

    // Assert
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Failed to connect to MongoDB"));
}

#[cfg(test)]
mod rocket_tests {
    use super::*;
    use crate::config::database;
    use rocket::local::blocking::Client;
    use rocket::{Build, Rocket};

    async fn setup_rocket() -> Rocket<Build> {
        rocket::build().attach(database::init())
    }

    #[tokio::test]
    async fn test_mongodb_fairing_success() {
        // Arrange
        setup();
        env::set_var("MONGODB_URI", "mongodb://localhost:27017");
        env::set_var("MONGODB_NAME", "test_db");

        // Act
        let rocket = setup_rocket().await;
        let client = Client::tracked(rocket).expect("valid rocket instance");

        // Assert
        let mongodb = client
            .rocket()
            .state::<MongoDB>()
            .expect("MongoDB should be managed");
        assert_eq!(mongodb.db.name(), "test_db");
    }

    #[tokio::test]
    #[should_panic(expected = "Failed to initialize MongoDB")]
    async fn test_mongodb_fairing_failure() {
        // Arrange
        setup();
        // No environment variables set

        // Act & Assert
        let _ = setup_rocket().await;
        // Should panic with "Failed to initialize MongoDB"
    }
}

#[tokio::test]
async fn test_mongodb_connection_verification() {
    // Arrange
    setup();
    env::set_var("MONGODB_URI", "mongodb://localhost:27017");

    // Act
    let result = MongoDB::init().await;

    // Assert
    assert!(result.is_ok());
    let mongodb = result.unwrap();

    // Verify that we can list databases
    let db_names = mongodb
        .client
        .list_database_names(None, None)
        .await
        .expect("Should be able to list databases");
    assert!(!db_names.is_empty());
}

#[tokio::test]
async fn test_mongodb_client_clone() {
    // Arrange
    setup();
    env::set_var("MONGODB_URI", "mongodb://localhost:27017");

    // Act
    let result = MongoDB::init().await;

    // Assert
    assert!(result.is_ok());
    let mongodb = result.unwrap();

    // Verify that client can be cloned
    let _cloned_client = mongodb.client.clone();
    assert!(true, "Client should be cloneable");
}

#[tokio::test]
async fn test_mongodb_empty_database_name() {
    // Arrange
    setup();
    env::set_var("MONGODB_URI", "mongodb://localhost:27017");
    env::set_var("MONGODB_NAME", "");

    // Act
    let result = MongoDB::init().await;

    // Assert
    assert!(result.is_ok());
    let mongodb = result.unwrap();
    assert_eq!(mongodb.db.name(), ""); // Empty database name should be allowed
}

#[tokio::test]
async fn test_mongodb_special_characters_in_name() {
    // Arrange
    setup();
    env::set_var("MONGODB_URI", "mongodb://localhost:27017");
    env::set_var("MONGODB_NAME", "test@db#123");

    // Act
    let result = MongoDB::init().await;

    // Assert
    assert!(result.is_ok());
    let mongodb = result.unwrap();
    assert_eq!(mongodb.db.name(), "test@db#123");
}
