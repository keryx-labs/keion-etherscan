use keion_etherscan::{EtherscanError};
use keion_etherscan::error::validation::{
    validate_address, validate_tx_hash, validate_block_hash, normalize_address
};

#[test]
fn test_error_display() {
    let err = EtherscanError::MissingApiKey;
    assert_eq!(err.to_string(), "API key is required but not provided");

    let err = EtherscanError::Http {
        status: 404,
        message: "Not Found".to_string(),
    };
    assert_eq!(err.to_string(), "HTTP error 404: Not Found");

    let err = EtherscanError::Api {
        message: "Invalid address".to_string(),
        result: Some("0".to_string()),
    };
    assert_eq!(err.to_string(), "API error: Invalid address (result: 0)");
}

#[test]
fn test_error_categories() {
    assert_eq!(EtherscanError::MissingApiKey.category(), "configuration");
    assert_eq!(EtherscanError::Request("test".to_string()).category(), "network");
    assert_eq!(EtherscanError::Parse("test".to_string()).category(), "parsing");
    assert_eq!(EtherscanError::InvalidAddress("bad".to_string()).category(), "validation");
    assert_eq!(EtherscanError::RateLimit { retry_after: None, message: "limit".to_string() }.category(), "rate_limit");
}

#[test]
fn test_retryable_errors() {
    assert!(EtherscanError::Request("timeout".to_string()).is_retryable());
    assert!(EtherscanError::Http { status: 500, message: "server error".to_string() }.is_retryable());
    assert!(EtherscanError::Http { status: 502, message: "bad gateway".to_string() }.is_retryable());
    assert!(EtherscanError::RateLimit { retry_after: Some(60), message: "limit".to_string() }.is_retryable());
    assert!(EtherscanError::Timeout("timeout".to_string()).is_retryable());

    // Non-retryable errors
    assert!(!EtherscanError::MissingApiKey.is_retryable());
    assert!(!EtherscanError::InvalidAddress("bad".to_string()).is_retryable());
    assert!(!EtherscanError::Http { status: 400, message: "bad request".to_string() }.is_retryable());
    assert!(!EtherscanError::Http { status: 404, message: "not found".to_string() }.is_retryable());
}

#[test]
fn test_error_constructors() {
    let api_err = EtherscanError::api("Something went wrong");
    match api_err {
        EtherscanError::Api { message, result } => {
            assert_eq!(message, "Something went wrong");
            assert_eq!(result, None);
        },
        _ => panic!("Expected API error"),
    }

    let api_err_with_result = EtherscanError::api_with_result("Failed", "0");
    match api_err_with_result {
        EtherscanError::Api { message, result } => {
            assert_eq!(message, "Failed");
            assert_eq!(result, Some("0".to_string()));
        },
        _ => panic!("Expected API error with result"),
    }

    let rate_limit_err = EtherscanError::rate_limit("Too many requests", Some(60));
    match rate_limit_err {
        EtherscanError::RateLimit { message, retry_after } => {
            assert_eq!(message, "Too many requests");
            assert_eq!(retry_after, Some(60));
        },
        _ => panic!("Expected rate limit error"),
    }
}

#[test]
fn test_address_validation() {
    // Valid addresses
    assert!(validate_address("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6").is_ok());
    assert!(validate_address("0x742D35Cc6634C0532925a3b8d19389c4D5e1e4a6").is_ok());
    assert!(validate_address("0x0000000000000000000000000000000000000000").is_ok());

    // Invalid addresses
    assert!(validate_address("742d35cc6634c0532925a3b8d19389c4d5e1e4a6").is_err()); // missing 0x
    assert!(validate_address("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a").is_err()); // too short
    assert!(validate_address("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6z").is_err()); // invalid hex
    assert!(validate_address("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6a").is_err()); // too long
    assert!(validate_address("").is_err()); // empty
    assert!(validate_address("0x").is_err()); // just prefix
}

#[test]
fn test_tx_hash_validation() {
    // Valid hash
    assert!(validate_tx_hash("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef").is_ok());
    assert!(validate_tx_hash("0x1234567890ABCDEF1234567890abcdef1234567890abcdef1234567890abcdef").is_ok());

    // Invalid hashes
    assert!(validate_tx_hash("1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef").is_err()); // missing 0x
    assert!(validate_tx_hash("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcde").is_err()); // too short
    assert!(validate_tx_hash("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdeg").is_err()); // invalid hex
    assert!(validate_tx_hash("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdefa").is_err()); // too long
    assert!(validate_tx_hash("").is_err()); // empty
}

#[test]
fn test_block_hash_validation() {
    // Block hash validation should work the same as tx hash validation
    assert!(validate_block_hash("0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef").is_ok());
    assert!(validate_block_hash("invalid").is_err());
}

#[test]
fn test_address_normalization() {
    let addr = "0x742D35Cc6634C0532925a3b8d19389c4D5e1e4a6";
    let normalized = normalize_address(addr).unwrap();
    assert_eq!(normalized, "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6");

    // Test that already lowercase addresses remain unchanged
    let lower_addr = "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6";
    let normalized_lower = normalize_address(lower_addr).unwrap();
    assert_eq!(normalized_lower, lower_addr);

    // Test that invalid addresses still fail
    assert!(normalize_address("invalid").is_err());
}

#[test]
fn test_error_from_conversions() {
    // Test conversion from reqwest::Error (simulated)
    let json_err = serde_json::from_str::<i32>("invalid json");
    let etherscan_err: EtherscanError = json_err.unwrap_err().into();
    match etherscan_err {
        EtherscanError::Parse(_) => {}, // Expected
        _ => panic!("Expected parse error"),
    }
}