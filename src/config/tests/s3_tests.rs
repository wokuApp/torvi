use crate::config::s3::S3Config;
use std::env;

fn setup() {
    env::remove_var("AWS_REGION");
    env::remove_var("AWS_ACCESS_KEY_ID");
    env::remove_var("AWS_SECRET_ACCESS_KEY");
    env::remove_var("AWS_S3_BUCKET");
}

#[test]
fn test_s3_config_success() {
    // Arrange
    setup();
    env::set_var("AWS_REGION", "us-east-1");
    env::set_var("AWS_ACCESS_KEY_ID", "test-key-id");
    env::set_var("AWS_SECRET_ACCESS_KEY", "test-secret-key");
    env::set_var("AWS_S3_BUCKET", "test-bucket");

    // Act
    let result = S3Config::from_env();

    // Assert
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.region, "us-east-1");
    assert_eq!(config.access_key_id, "test-key-id");
    assert_eq!(config.secret_access_key, "test-secret-key");
    assert_eq!(config.bucket, "test-bucket");
}

#[test]
fn test_s3_config_missing_region() {
    // Arrange
    setup();
    env::set_var("AWS_ACCESS_KEY_ID", "test-key-id");
    env::set_var("AWS_SECRET_ACCESS_KEY", "test-secret-key");
    env::set_var("AWS_S3_BUCKET", "test-bucket");

    // Act
    let result = S3Config::from_env();

    // Assert
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Missing AWS_REGION environment variable"
    );
}

#[test]
fn test_s3_config_missing_access_key_id() {
    // Arrange
    setup();
    env::set_var("AWS_REGION", "us-east-1");
    env::set_var("AWS_SECRET_ACCESS_KEY", "test-secret-key");
    env::set_var("AWS_S3_BUCKET", "test-bucket");

    // Act
    let result = S3Config::from_env();

    // Assert
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Missing AWS_ACCESS_KEY_ID environment variable"
    );
}

#[test]
fn test_s3_config_missing_secret_access_key() {
    // Arrange
    setup();
    env::set_var("AWS_REGION", "us-east-1");
    env::set_var("AWS_ACCESS_KEY_ID", "test-key-id");
    env::set_var("AWS_S3_BUCKET", "test-bucket");

    // Act
    let result = S3Config::from_env();

    // Assert
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Missing AWS_SECRET_ACCESS_KEY environment variable"
    );
}

#[test]
fn test_s3_config_missing_bucket() {
    // Arrange
    setup();
    env::set_var("AWS_REGION", "us-east-1");
    env::set_var("AWS_ACCESS_KEY_ID", "test-key-id");
    env::set_var("AWS_SECRET_ACCESS_KEY", "test-secret-key");

    // Act
    let result = S3Config::from_env();

    // Assert
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err(),
        "Missing AWS_S3_BUCKET environment variable"
    );
}

#[test]
fn test_s3_config_empty_values() {
    // Arrange
    setup();
    env::set_var("AWS_REGION", "");
    env::set_var("AWS_ACCESS_KEY_ID", "");
    env::set_var("AWS_SECRET_ACCESS_KEY", "");
    env::set_var("AWS_S3_BUCKET", "");

    // Act
    let result = S3Config::from_env();

    // Assert
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.region, "");
    assert_eq!(config.access_key_id, "");
    assert_eq!(config.secret_access_key, "");
    assert_eq!(config.bucket, "");
}

#[test]
fn test_s3_config_whitespace_values() {
    // Arrange
    setup();
    env::set_var("AWS_REGION", "  us-east-1  ");
    env::set_var("AWS_ACCESS_KEY_ID", "  test-key-id  ");
    env::set_var("AWS_SECRET_ACCESS_KEY", "  test-secret-key  ");
    env::set_var("AWS_S3_BUCKET", "  test-bucket  ");

    // Act
    let result = S3Config::from_env();

    // Assert
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.region, "  us-east-1  ");
    assert_eq!(config.access_key_id, "  test-key-id  ");
    assert_eq!(config.secret_access_key, "  test-secret-key  ");
    assert_eq!(config.bucket, "  test-bucket  ");
}

#[test]
fn test_s3_config_special_characters() {
    // Arrange
    setup();
    env::set_var("AWS_REGION", "us-east-1");
    env::set_var("AWS_ACCESS_KEY_ID", "AKIA@key#123");
    env::set_var("AWS_SECRET_ACCESS_KEY", "secret$key%456");
    env::set_var("AWS_S3_BUCKET", "my-bucket.test-789");

    // Act
    let result = S3Config::from_env();

    // Assert
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.region, "us-east-1");
    assert_eq!(config.access_key_id, "AKIA@key#123");
    assert_eq!(config.secret_access_key, "secret$key%456");
    assert_eq!(config.bucket, "my-bucket.test-789");
}

#[cfg(test)]
mod rocket_tests {
    use super::*;
    use crate::config::s3;
    use rocket::local::blocking::Client;
    use rocket::{Build, Rocket};

    async fn setup_rocket() -> Rocket<Build> {
        rocket::build().attach(s3::init())
    }

    #[tokio::test]
    async fn test_s3_fairing_success() {
        // Arrange
        setup();
        env::set_var("AWS_REGION", "us-east-1");
        env::set_var("AWS_ACCESS_KEY_ID", "test-key-id");
        env::set_var("AWS_SECRET_ACCESS_KEY", "test-secret-key");
        env::set_var("AWS_S3_BUCKET", "test-bucket");

        // Act
        let rocket = setup_rocket().await;
        let client = Client::tracked(rocket).expect("valid rocket instance");

        // Assert
        let config = client
            .rocket()
            .state::<S3Config>()
            .expect("S3 config should be managed");
        assert_eq!(config.region, "us-east-1");
        assert_eq!(config.access_key_id, "test-key-id");
        assert_eq!(config.secret_access_key, "test-secret-key");
        assert_eq!(config.bucket, "test-bucket");
    }

    #[tokio::test]
    #[should_panic(expected = "Failed to initialize S3 configuration")]
    async fn test_s3_fairing_failure() {
        // Arrange
        setup();
        // No environment variables set

        // Act & Assert
        let _ = setup_rocket().await;
        // Should panic with "Failed to initialize S3 configuration"
    }
}
