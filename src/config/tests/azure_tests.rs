use crate::config::azure::AzureConfig;
use std::env;

fn setup() {
    env::remove_var("AZURE_STORAGE_ACCOUNT");
    env::remove_var("AZURE_STORAGE_KEY");
    env::remove_var("AZURE_STORAGE_CONTAINER");
}

#[test]
fn test_azure_config_success() {
    // Arrange
    setup();
    env::set_var("AZURE_STORAGE_ACCOUNT", "test-account");
    env::set_var("AZURE_STORAGE_KEY", "test-key");
    env::set_var("AZURE_STORAGE_CONTAINER", "test-container");

    // Act
    let result = AzureConfig::from_env();

    // Assert
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.storage_account, "test-account");
    assert_eq!(config.access_key, "test-key");
    assert_eq!(config.container, "test-container");
}

#[test]
fn test_azure_config_missing_account() {
    // Arrange
    setup();
    env::set_var("AZURE_STORAGE_KEY", "test-key");
    env::set_var("AZURE_STORAGE_CONTAINER", "test-container");

    // Act
    let result = AzureConfig::from_env();

    // Assert
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Missing AZURE_STORAGE_ACCOUNT environment variable"
    );
}

#[test]
fn test_azure_config_missing_key() {
    // Arrange
    setup();
    env::set_var("AZURE_STORAGE_ACCOUNT", "test-account");
    env::set_var("AZURE_STORAGE_CONTAINER", "test-container");

    // Act
    let result = AzureConfig::from_env();

    // Assert
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Missing AZURE_STORAGE_KEY environment variable"
    );
}

#[test]
fn test_azure_config_missing_container() {
    // Arrange
    setup();
    env::set_var("AZURE_STORAGE_ACCOUNT", "test-account");
    env::set_var("AZURE_STORAGE_KEY", "test-key");

    // Act
    let result = AzureConfig::from_env();

    // Assert
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Missing AZURE_STORAGE_CONTAINER environment variable"
    );
}

#[test]
fn test_azure_config_empty_values() {
    // Arrange
    setup();
    env::set_var("AZURE_STORAGE_ACCOUNT", "");
    env::set_var("AZURE_STORAGE_KEY", "");
    env::set_var("AZURE_STORAGE_CONTAINER", "");

    // Act
    let result = AzureConfig::from_env();

    // Assert
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.storage_account, "");
    assert_eq!(config.access_key, "");
    assert_eq!(config.container, "");
}

#[test]
fn test_azure_config_whitespace_values() {
    // Arrange
    setup();
    env::set_var("AZURE_STORAGE_ACCOUNT", "  test-account  ");
    env::set_var("AZURE_STORAGE_KEY", "  test-key  ");
    env::set_var("AZURE_STORAGE_CONTAINER", "  test-container  ");

    // Act
    let result = AzureConfig::from_env();

    // Assert
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.storage_account, "  test-account  ");
    assert_eq!(config.access_key, "  test-key  ");
    assert_eq!(config.container, "  test-container  ");
}

#[test]
fn test_azure_config_special_characters() {
    // Arrange
    setup();
    env::set_var("AZURE_STORAGE_ACCOUNT", "test@account#123");
    env::set_var("AZURE_STORAGE_KEY", "test$key%456");
    env::set_var("AZURE_STORAGE_CONTAINER", "test&container*789");

    // Act
    let result = AzureConfig::from_env();

    // Assert
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.storage_account, "test@account#123");
    assert_eq!(config.access_key, "test$key%456");
    assert_eq!(config.container, "test&container*789");
}

#[cfg(test)]
mod rocket_tests {
    use super::*;
    use crate::config::azure;
    use rocket::local::blocking::Client;
    use rocket::{Build, Rocket};

    async fn setup_rocket() -> Rocket<Build> {
        rocket::build().attach(azure::init())
    }

    #[tokio::test]
    async fn test_azure_fairing_success() {
        // Arrange
        setup();
        env::set_var("AZURE_STORAGE_ACCOUNT", "test-account");
        env::set_var("AZURE_STORAGE_KEY", "test-key");
        env::set_var("AZURE_STORAGE_CONTAINER", "test-container");

        // Act
        let rocket = setup_rocket().await;
        let client = Client::tracked(rocket).expect("valid rocket instance");

        // Assert
        let config = client
            .rocket()
            .state::<AzureConfig>()
            .expect("Azure config should be managed");
        assert_eq!(config.storage_account, "test-account");
        assert_eq!(config.access_key, "test-key");
        assert_eq!(config.container, "test-container");
    }

    #[tokio::test]
    #[should_panic(expected = "Failed to initialize Azure configuration")]
    async fn test_azure_fairing_failure() {
        // Arrange
        setup();
        // No environment variables set

        // Act & Assert
        let _ = setup_rocket().await;
        // Should panic with "Failed to initialize Azure configuration"
    }
}
