//! These tests can fail if run in parallel!
//! Use: `cargo test -- --test-threads=1` to run them in sequence.

const CONFIG_FILE: &'static str = "target/test_config_from_tests.toml";
const CONFIG_CONTENT: &'static str = r#"api_key = "your_api_key"
api_password = "your_api_password"
customer_number = 1234567890"#;

fn assert_config(config: crate::Config) {
    assert_eq!(config.api_key, "your_api_key");
    assert_eq!(config.api_password, "your_api_password");
    assert_eq!(config.customer_number, 1234567890);
}

fn set_env_vars(config_str: &str) {
    let s: Vec<&str> = config_str.split("\n").collect();
    std::env::set_var(
        "API_KEY",
        s[0].split(" = ").nth(1).unwrap().replace("\"", ""),
    );
    std::env::set_var(
        "API_SECRET",
        s[1].split(" = ").nth(1).unwrap().replace("\"", ""),
    );
    std::env::set_var("CUSTOMER_NUMBER", s[2].split(" = ").nth(1).unwrap());
}

fn set_env_file(path: &str) {
    std::env::set_var("CONFIG_PATH", path);
}

fn write_config(path: &str, content: &str) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(tokio::fs::write(path, content))
        .expect("THIS IS NOT A TEST FAILURE; Writing the test file failed!");
}

#[test]
fn read_from_file() {
    // --- Setup ---
    let rt = tokio::runtime::Runtime::new().unwrap();
    write_config(CONFIG_FILE, CONFIG_CONTENT);

    // --- Test ---
    let config = rt
        .block_on(crate::Config::read_from_file(CONFIG_FILE))
        .expect("test config reading failed");

    assert_config(config);
}

#[test]
fn read_from_file_in_env() {
    // --- Setup ---
    let rt = tokio::runtime::Runtime::new().unwrap();
    write_config(CONFIG_FILE, CONFIG_CONTENT);
    set_env_file(CONFIG_FILE);

    // --- Test ---
    let config = rt
        .block_on(crate::Config::read_from_file_in_env())
        .expect("test config reading failed");

    assert_config(config);
}

#[test]
fn read_from_env() {
    // --- Setup ---
    let rt = tokio::runtime::Runtime::new().unwrap();
    write_config(CONFIG_FILE, CONFIG_CONTENT);
    set_env_vars(CONFIG_CONTENT);

    // --- Test ---
    let config = rt
        .block_on(crate::Config::read_from_env())
        .expect("test config reading failed");

    assert_config(config);
}

#[test]
fn read_order_env_over_env_file() {
    // --- Setup ---
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Important: Change the 2nd (in this case: env-file) to different values from the actual correct values.
    // The assertion at the end will test for the correct values (see above).
    // If it finds the changed values it wil fail.
    write_config(
        CONFIG_FILE,
        r#"api_key = "THIS IS"
api_password = "TOTALLY DIFFERENT"
customer_number = 9876543210"#,
    );
    set_env_vars(CONFIG_CONTENT);
    set_env_file(CONFIG_FILE);

    // --- Test ---
    let config = rt
        .block_on(crate::Config::read())
        .expect("test config reading failed");

    assert_config(config);
}

#[test]
fn read_order_env_file_over_default_path() {
    // --- Setup ---
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Important: Change the 2nd (in this case: default-file) to different values from the actual correct values.
    // The assertion at the end will test for the correct values (see above).
    // If it finds the changed values it wil fail.
    write_config(CONFIG_FILE, CONFIG_CONTENT);
    write_config(
        "config.toml",
        r#"api_key = "THIS IS"
api_password = "TOTALLY DIFFERENT"
customer_number = 12345"#,
    );
    set_env_file(CONFIG_FILE);

    // --- Test ---
    let config = rt
        .block_on(crate::Config::read())
        .expect("test config reading failed");

    assert_config(config);
}
