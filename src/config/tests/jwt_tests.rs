use crate::config::jwt::JwtConfig;
use std::env;

fn setup() {
    env::remove_var("JWT_SECRET");
}

#[test]
fn test_jwt_config_success() {
    // Arrange
    setup();
    env::set_var("JWT_SECRET", "test-secret-key");

    // Act
    let result = JwtConfig::from_env();

    // Assert
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.secret, "test-secret-key");
}

#[test]
fn test_jwt_config_missing_secret() {
    // Arrange
    setup();

    // Act
    let result = JwtConfig::from_env();

    // Assert
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Missing JWT_SECRET environment variable"
    );
}

#[test]
fn test_jwt_config_empty_secret() {
    // Arrange
    setup();
    env::set_var("JWT_SECRET", "");

    // Act
    let result = JwtConfig::from_env();

    // Assert
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.secret, "");
}

#[test]
fn test_jwt_config_whitespace_secret() {
    // Arrange
    setup();
    env::set_var("JWT_SECRET", "  test-secret-key  ");

    // Act
    let result = JwtConfig::from_env();

    // Assert
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.secret, "  test-secret-key  ");
}

#[test]
fn test_jwt_config_special_characters() {
    // Arrange
    setup();
    env::set_var("JWT_SECRET", "test@secret#key$123");

    // Act
    let result = JwtConfig::from_env();

    // Assert
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.secret, "test@secret#key$123");
}

#[cfg(test)]
mod rocket_tests {
    use super::*;
    use crate::config::jwt;
    use rocket::local::blocking::Client;
    use rocket::{Build, Rocket};

    async fn setup_rocket() -> Rocket<Build> {
        rocket::build().attach(jwt::init())
    }

    #[tokio::test]
    async fn test_jwt_fairing_success() {
        // Arrange
        setup();
        env::set_var("JWT_SECRET", "test-secret-key");

        // Act
        let rocket = setup_rocket().await;
        let client = Client::tracked(rocket).expect("valid rocket instance");

        // Assert
        let config = client
            .rocket()
            .state::<JwtConfig>()
            .expect("JWT config should be managed");
        assert_eq!(config.secret, "test-secret-key");
    }

    #[tokio::test]
    #[should_panic(expected = "Failed to initialize JWT configuration")]
    async fn test_jwt_fairing_failure() {
        // Arrange
        setup();
        // No environment variables set

        // Act & Assert
        let _ = setup_rocket().await;
        // Should panic with "Failed to initialize JWT configuration"
    }
}

#[test]
fn test_jwt_config_unicode_characters() {
    // Arrange
    setup();
    env::set_var("JWT_SECRET", "测试密钥🔑");

    // Act
    let result = JwtConfig::from_env();

    // Assert
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.secret, "测试密钥🔑");
}

#[test]
fn test_jwt_config_long_secret() {
    // Arrange
    setup();
    let long_secret = "a".repeat(1000);
    env::set_var("JWT_SECRET", &long_secret);

    // Act
    let result = JwtConfig::from_env();

    // Assert
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.secret, long_secret);
}

#[test]
fn test_jwt_config_newline_characters() {
    // Arrange
    setup();
    env::set_var("JWT_SECRET", "test\nsecret\nkey");

    // Act
    let result = JwtConfig::from_env();

    // Assert
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.secret, "test\nsecret\nkey");
}
