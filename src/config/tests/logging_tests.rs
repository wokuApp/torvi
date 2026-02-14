use serial_test::serial;

use crate::config::logging::build_env_filter;

#[test]
#[serial]
fn test_env_filter_default_parses() {
    std::env::remove_var("RUST_LOG");
    let filter = build_env_filter();
    let debug = format!("{}", filter);
    assert!(debug.contains("info"));
}

#[test]
#[serial]
fn test_env_filter_custom_parses() {
    std::env::set_var("RUST_LOG", "warn,torvi=trace");
    let filter = build_env_filter();
    let debug = format!("{}", filter);
    assert!(debug.contains("warn"));
    std::env::remove_var("RUST_LOG");
}
