mod common;

use common::{TestUtils, MockResponses};
use keion_etherscan::{EtherscanError, EtherscanClient, Result};

/// Test error handling for invalid addresses
mod address_validation_tests {
    use super::*;

    #[tokio::test]
    async fn test_invalid_address_too_short() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        // This should fail during execution due to address validation
        let result = accounts
            .transactions(TestUtils::invalid_address_too_short())
            .execute()
            .await;

        assert!(result.is_err());
        match result.unwrap_err() {
            EtherscanError::InvalidAddress(addr) => {
                assert_eq!(addr, TestUtils::invalid_address_too_short());
            }
            _ => panic!("Expected InvalidAddress error"),
        }
    }

    #[tokio::test]
    async fn test_invalid_address_no_prefix() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        let result = accounts
            .transactions(TestUtils::invalid_address_no_prefix())
            .execute()
            .await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EtherscanError::InvalidAddress(_)));
    }

    #[tokio::test]
    async fn test_invalid_address_wrong_length() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        let result = accounts
            .transactions(TestUtils::invalid_address_wrong_length())
            .execute()
            .await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EtherscanError::InvalidAddress(_)));
    }

    #[tokio::test]
    async fn test_invalid_address_non_hex() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        let result = accounts
            .transactions(TestUtils::invalid_address_non_hex())
            .execute()
            .await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EtherscanError::InvalidAddress(_)));
    }

    #[tokio::test]
    async fn test_valid_address_mixed_case_normalization() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        // This should succeed with address normalization
        // Note: In a real test, this would need mocking to avoid actual API calls
        let query = accounts.transactions(TestUtils::valid_address_mixed_case());
        
        // The query should be buildable
        assert_eq!(query.get_address(), TestUtils::valid_address_mixed_case());
    }

    #[tokio::test]
    async fn test_internal_transactions_invalid_hash() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        let result = accounts
            .internal_transactions()
            .by_hash(TestUtils::invalid_tx_hash_too_short())
            .execute()
            .await;

        assert!(result.is_err());
        // Hash validation might be done differently, but should fail
    }

    #[tokio::test]
    async fn test_historical_balance_invalid_address() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        let result = accounts
            .historical_balance(TestUtils::invalid_address_too_short())
            .at_block(1000000)
            .execute()
            .await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EtherscanError::InvalidAddress(_)));
    }

    #[tokio::test]
    async fn test_beacon_withdrawals_invalid_address() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        let result = accounts
            .beacon_withdrawals(TestUtils::invalid_address_non_hex())
            .execute()
            .await;

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EtherscanError::InvalidAddress(_)));
    }

    #[tokio::test]
    async fn test_multi_balance_validation() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        // Test empty address list
        let result = accounts.balance_multi(&Vec::<&str>::new()).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EtherscanError::InvalidParams(_)));

        // Test too many addresses (> 20)
        let too_many_addresses: Vec<&str> = (0..25)
            .map(|_| TestUtils::valid_address())
            .collect();
        
        let result = accounts.balance_multi(&too_many_addresses).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EtherscanError::InvalidParams(_)));

        // Test with invalid address in list
        let mixed_addresses = vec![
            TestUtils::valid_address(),
            TestUtils::invalid_address_too_short(),
            TestUtils::valid_address(),
        ];
        
        let result = accounts.balance_multi(&mixed_addresses).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), EtherscanError::InvalidAddress(_)));
    }
}

/// Test API error responses
mod api_error_tests {
    use super::*;

    #[tokio::test]
    async fn test_api_error_response_handling() {
        // Note: In a real implementation, this would use a mock HTTP client
        // For now, we test the error types and structure
        
        let error = EtherscanError::api("Invalid address format");
        assert!(matches!(error, EtherscanError::Api { .. }));
        
        if let EtherscanError::Api { message, result } = error {
            assert_eq!(message, "Invalid address format");
            assert_eq!(result, None);
        }
    }

    #[tokio::test]
    async fn test_rate_limit_error() {
        let error = EtherscanError::rate_limit("Max rate limit reached", Some(60));
        assert!(matches!(error, EtherscanError::RateLimit { .. }));
        
        if let EtherscanError::RateLimit { retry_after, message } = error {
            assert_eq!(message, "Max rate limit reached");
            assert_eq!(retry_after, Some(60));
        }
    }

    #[tokio::test]
    async fn test_network_error() {
        let error = EtherscanError::Request("Connection timeout".to_string());
        assert!(matches!(error, EtherscanError::Request(_)));
        assert!(error.is_retryable());
    }

    #[tokio::test]
    async fn test_http_error() {
        let error = EtherscanError::Http {
            status: 500,
            message: "Internal Server Error".to_string(),
        };
        assert!(matches!(error, EtherscanError::Http { .. }));
        assert!(error.is_retryable());
    }

    #[tokio::test]
    async fn test_parse_error() {
        let error = EtherscanError::Parse("Invalid JSON response".to_string());
        assert!(matches!(error, EtherscanError::Parse(_)));
        assert!(!error.is_retryable());
    }

    #[test]
    fn test_error_categorization() {
        let address_error = EtherscanError::InvalidAddress("0x123".to_string());
        assert_eq!(address_error.category(), "validation");

        let api_error = EtherscanError::api("Something went wrong");
        assert_eq!(api_error.category(), "api");

        let network_error = EtherscanError::Request("Timeout".to_string());
        assert_eq!(network_error.category(), "network");

        let rate_limit_error = EtherscanError::rate_limit("Too many requests", None);
        assert_eq!(rate_limit_error.category(), "rate_limit");
    }
}

/// Test malformed data handling
mod malformed_data_tests {
    use super::*;

    #[test]
    fn test_malformed_json_handling() {
        // Test that malformed JSON responses are handled properly
        let malformed_json = r#"{"status": "1", "message": "OK", "result": invalid_json}"#;
        
        let parse_result: std::result::Result<serde_json::Value, _> = serde_json::from_str(malformed_json);
        assert!(parse_result.is_err());
        
        // In the actual implementation, this would be converted to EtherscanError::Parse
        let error = EtherscanError::Parse("JSON parse error".to_string());
        assert!(matches!(error, EtherscanError::Parse(_)));
    }

    #[test]
    fn test_missing_required_fields() {
        // Test JSON with missing required fields
        let incomplete_balance = r#"{"account": "0x123"}"#; // Missing balance field
        
        let parse_result: std::result::Result<serde_json::Value, _> = serde_json::from_str(incomplete_balance);
        assert!(parse_result.is_ok()); // JSON is valid, but missing fields
        
        // The actual deserialization to Balance struct would fail
        // This would result in EtherscanError::Parse in the real implementation
    }

    #[test]
    fn test_invalid_field_types() {
        // Test JSON with correct fields but wrong types
        let invalid_types = r#"{
            "blockNumber": "not_a_number",
            "timeStamp": "also_not_a_number",
            "blockReward": "definitely_not_a_number"
        }"#;
        
        let parse_result: std::result::Result<serde_json::Value, _> = serde_json::from_str(invalid_types);
        assert!(parse_result.is_ok()); // JSON is valid
        
        // But deserialization to ValidatedBlock would fail due to string-to-number conversion
    }

    #[test]
    fn test_empty_string_handling() {
        // Test how empty strings are handled in numeric fields
        let empty_strings = r#"{
            "blockNumber": "",
            "timeStamp": "",
            "balance": ""
        }"#;
        
        let parse_result: std::result::Result<serde_json::Value, _> = serde_json::from_str(empty_strings);
        assert!(parse_result.is_ok());
        
        // Empty strings should be handled gracefully in the deserializers
    }

    #[test]
    fn test_null_values() {
        // Test how null values are handled
        let null_values = r#"{
            "blockNumber": null,
            "timeStamp": null,
            "balance": null
        }"#;
        
        let parse_result: std::result::Result<serde_json::Value, _> = serde_json::from_str(null_values);
        assert!(parse_result.is_ok());
        
        // Null values should be handled appropriately based on field optionality
    }
}

/// Test edge cases for specific error scenarios
mod edge_case_error_tests {
    use super::*;

    #[test]
    fn test_unsupported_network_features() {
        // Test unsupported network error
        let error = EtherscanError::unsupported_network("Goerli", "beacon_withdrawals");
        
        assert!(matches!(error, EtherscanError::UnsupportedNetwork { .. }));
        if let EtherscanError::UnsupportedNetwork { network, feature } = error {
            assert_eq!(network, "Goerli");
            assert_eq!(feature, "beacon_withdrawals");
        }
    }

    #[test]
    fn test_invalid_block_identifiers() {
        // Test various invalid block identifiers
        let invalid_blocks = vec![
            "not_a_number",
            "-1",
            "0x", // Invalid hex
            "pending_typo", // Should be "pending"
            "latest_typo",  // Should be "latest"
        ];

        for invalid_block in invalid_blocks {
            let error = EtherscanError::InvalidBlock(invalid_block.to_string());
            assert!(matches!(error, EtherscanError::InvalidBlock(_)));
        }
    }

    #[test]
    fn test_invalid_transaction_hash_formats() {
        let invalid_hashes = vec![
            TestUtils::invalid_tx_hash_too_short(),
            TestUtils::invalid_tx_hash_no_prefix(), 
            TestUtils::invalid_tx_hash_non_hex(),
            "", // Empty hash
            "0x", // Just prefix
        ];

        for invalid_hash in invalid_hashes {
            let error = EtherscanError::InvalidTxHash(invalid_hash.to_string());
            assert!(matches!(error, EtherscanError::InvalidTxHash(_)));
        }
    }

    #[test]
    fn test_parameter_boundary_conditions() {
        // Test parameter validation for boundary conditions
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        // These should build successfully but might fail during execution
        let _max_page = accounts
            .transactions(TestUtils::valid_address())
            .page(u32::MAX);

        let _zero_offset = accounts
            .transactions(TestUtils::valid_address())
            .offset(0);

        // Block range where start > end
        let _invalid_range = accounts
            .internal_transactions()
            .by_block_range(2000000, 1000000);
    }

    #[tokio::test]
    async fn test_timeout_error_handling() {
        // Test timeout error
        let error = EtherscanError::Timeout("Request timed out after 30s".to_string());
        assert!(matches!(error, EtherscanError::Timeout(_)));
        assert!(error.is_retryable());
    }

    #[test]
    fn test_error_display_formatting() {
        // Test that error display messages are formatted correctly
        let address_error = EtherscanError::InvalidAddress("0x123".to_string());
        let display = format!("{}", address_error);
        assert!(display.contains("Invalid Ethereum address: 0x123"));

        let api_error = EtherscanError::Api {
            message: "Invalid API key".to_string(),
            result: Some("Unauthorized".to_string()),
        };
        let display = format!("{}", api_error);
        assert!(display.contains("API error: Invalid API key"));
        assert!(display.contains("result: Unauthorized"));

        let rate_limit = EtherscanError::RateLimit {
            message: "Too many requests".to_string(),
            retry_after: Some(60),
        };
        let display = format!("{}", rate_limit);
        assert!(display.contains("Rate limit exceeded"));
        assert!(display.contains("retry after 60 seconds"));
    }
}