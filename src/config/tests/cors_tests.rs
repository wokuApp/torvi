use serial_test::serial;

use crate::config::cors::build_cors_options;

#[test]
#[serial]
fn test_cors_default_origin() {
    std::env::remove_var("CORS_ALLOWED_ORIGINS");
    let options = build_cors_options();
    assert!(options.to_cors().is_ok());
}

#[test]
#[serial]
fn test_cors_custom_origins() {
    std::env::set_var(
        "CORS_ALLOWED_ORIGINS",
        "https://example.com,https://app.example.com",
    );
    let options = build_cors_options();
    assert!(options.to_cors().is_ok());
    std::env::remove_var("CORS_ALLOWED_ORIGINS");
}

#[test]
#[serial]
fn test_cors_allows_common_methods() {
    std::env::remove_var("CORS_ALLOWED_ORIGINS");
    let options = build_cors_options();
    assert_eq!(options.allowed_methods.len(), 6);
}

#[test]
#[serial]
fn test_cors_allows_common_headers() {
    std::env::remove_var("CORS_ALLOWED_ORIGINS");
    let options = build_cors_options();
    assert!(!options.allowed_headers.is_all());
}

#[test]
#[serial]
fn test_cors_supports_credentials() {
    std::env::remove_var("CORS_ALLOWED_ORIGINS");
    let options = build_cors_options();
    assert!(options.allow_credentials);
}

#[test]
#[serial]
fn test_cors_trims_whitespace_in_origins() {
    std::env::set_var(
        "CORS_ALLOWED_ORIGINS",
        " https://example.com , https://app.example.com ",
    );
    let options = build_cors_options();
    assert!(options.to_cors().is_ok());
    std::env::remove_var("CORS_ALLOWED_ORIGINS");
}
