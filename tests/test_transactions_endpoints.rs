mod common;

use common::TestUtils;
use keion_etherscan::models::{ContractExecutionStatus, StringNumber, TransactionReceiptStatus, TransactionStatus, TxHash};

/// Test transaction status checking endpoint builders (non-async tests)
mod builder_tests {
    use super::*;

    #[test]
    fn test_batch_status_builder_creation() {
        let client = TestUtils::create_test_client();
        let transactions = client.transactions();

        let builder = transactions.batch_status();

        // Test initial state
        assert!(builder.get_tx_hashes().is_empty());
        assert!(builder.get_check_execution());
        assert!(builder.get_check_receipt());
    }

    #[test]
    fn test_batch_status_builder_single_transaction() {
        let client = TestUtils::create_test_client();
        let transactions = client.transactions();

        let tx_hash = TestUtils::valid_tx_hash();
        let builder = transactions.batch_status().transaction(tx_hash);

        assert_eq!(builder.get_tx_hashes().len(), 1);
        assert_eq!(builder.get_tx_hashes()[0].as_str(), tx_hash);
    }

    #[test]
    fn test_batch_status_builder_multiple_transactions() {
        let client = TestUtils::create_test_client();
        let transactions = client.transactions();

        let tx_hashes = vec![
            "0x1234567890123456789012345678901234567890123456789012345678901234",
            "0x5678901234567890123456789012345678901234567890123456789012345678",
            "0x9012345678901234567890123456789012345678901234567890123456789012",
        ];

        let builder = transactions.batch_status().transactions(&tx_hashes);

        assert_eq!(builder.get_tx_hashes().len(), 3);
        for (i, expected_hash) in tx_hashes.iter().enumerate() {
            assert_eq!(builder.get_tx_hashes()[i].as_str(), *expected_hash);
        }
    }

    #[test]
    fn test_batch_status_builder_execution_only() {
        let client = TestUtils::create_test_client();
        let transactions = client.transactions();

        let builder = transactions
            .batch_status()
            .transaction(TestUtils::valid_tx_hash())
            .execution_only();

        assert!(builder.get_check_execution());
        assert!(!builder.get_check_receipt());
    }

    #[test]
    fn test_batch_status_builder_receipt_only() {
        let client = TestUtils::create_test_client();
        let transactions = client.transactions();

        let builder = transactions
            .batch_status()
            .transaction(TestUtils::valid_tx_hash())
            .receipt_only();

        assert!(!builder.get_check_execution());
        assert!(builder.get_check_receipt());
    }

    #[test]
    fn test_batch_status_builder_chaining() {
        let client = TestUtils::create_test_client();
        let transactions = client.transactions();

        let builder = transactions
            .batch_status()
            .transaction("0x1234567890123456789012345678901234567890123456789012345678901234")
            .transaction("0x5678901234567890123456789012345678901234567890123456789012345678")
            .execution_only();

        assert_eq!(builder.get_tx_hashes().len(), 2);
        assert!(builder.get_check_execution());
        assert!(!builder.get_check_receipt());
    }
}

/// Test parameter validation
mod parameter_validation_tests {
    use super::*;

    #[test]
    fn test_valid_transaction_hash() {
        let client = TestUtils::create_test_client();
        let transactions = client.transactions();

        // These should not panic during builder creation
        let _builder1 = transactions
            .batch_status()
            .transaction(TestUtils::valid_tx_hash());

        let _builder2 = transactions
            .batch_status()
            .transaction("0xABCDEF1234567890123456789012345678901234567890123456789012345678");
    }

    #[test]
    fn test_mixed_case_transaction_hash_normalization() {
        let client = TestUtils::create_test_client();
        let transactions = client.transactions();

        let mixed_case_hash = "0xABCDef1234567890123456789012345678901234567890123456789012345678";
        let builder = transactions.batch_status().transaction(mixed_case_hash);

        // Should be normalized to lowercase
        assert_eq!(
            builder.get_tx_hashes()[0].as_str(),
            mixed_case_hash.to_lowercase()
        );
    }

    #[test]
    fn test_empty_transaction_list_batch() {
        let client = TestUtils::create_test_client();
        let transactions = client.transactions();

        let empty_vec: Vec<&str> = vec![];
        let builder = transactions.batch_status().transactions(&empty_vec);

        assert!(builder.get_tx_hashes().is_empty());
    }
}

/// Test model helper methods and logic
mod model_tests {
    use super::*;

    #[test]
    fn test_contract_execution_status_success() {
        let status = ContractExecutionStatus {
            is_error: StringNumber::from(0),
            error_description: String::new(),
        };

        assert!(status.is_successful());
        assert!(!status.is_failed());
        assert!(status.error_message().is_none());
    }

    #[test]
    fn test_contract_execution_status_failure() {
        let status = ContractExecutionStatus {
            is_error: StringNumber::from(1),
            error_description: "Bad jump destination".to_string(),
        };

        assert!(!status.is_successful());
        assert!(status.is_failed());
        assert_eq!(status.error_message(), Some("Bad jump destination"));
    }

    #[test]
    fn test_contract_execution_status_failure_empty_description() {
        let status = ContractExecutionStatus {
            is_error: StringNumber::from(1),
            error_description: String::new(),
        };

        assert!(!status.is_successful());
        assert!(status.is_failed());
        assert!(status.error_message().is_none());
    }

    #[test]
    fn test_transaction_receipt_status_success() {
        let status = TransactionReceiptStatus {
            status: StringNumber::from(1),
        };

        assert!(status.is_successful());
        assert!(!status.is_failed());
    }

    #[test]
    fn test_transaction_receipt_status_failure() {
        let status = TransactionReceiptStatus {
            status: StringNumber::from(0),
        };

        assert!(!status.is_successful());
        assert!(status.is_failed());
    }
}

/// Test combined transaction status logic
mod transaction_status_tests {
    use super::*;
    use keion_etherscan::models::{TransactionStatus, TxHash};

    #[test]
    fn test_transaction_status_new() {
        let tx_hash = "0x1234567890123456789012345678901234567890123456789012345678901234";
        let status = TransactionStatus::new(tx_hash);

        assert_eq!(status.tx_hash.as_str(), tx_hash);
        assert!(status.contract_execution.is_none());
        assert!(status.receipt_status.is_none());
        assert!(status.is_successful().is_none());
    }

    #[test]
    fn test_transaction_status_with_receipt_success() {
        let tx_hash = "0x1234567890123456789012345678901234567890123456789012345678901234";
        let mut status = TransactionStatus::new(tx_hash);

        status.receipt_status = Some(TransactionReceiptStatus {
            status: StringNumber::from(1),
        });

        assert_eq!(status.is_successful(), Some(true));
        assert!(status.status_description().contains("successful"));
    }

    #[test]
    fn test_transaction_status_with_receipt_failure() {
        let tx_hash = "0x1234567890123456789012345678901234567890123456789012345678901234";
        let mut status = TransactionStatus::new(tx_hash);

        status.receipt_status = Some(TransactionReceiptStatus {
            status: StringNumber::from(0),
        });

        assert_eq!(status.is_successful(), Some(false));
        assert!(status.status_description().contains("failed"));
    }

    #[test]
    fn test_transaction_status_with_execution_success() {
        let tx_hash = "0x1234567890123456789012345678901234567890123456789012345678901234";
        let mut status = TransactionStatus::new(tx_hash);

        status.contract_execution = Some(ContractExecutionStatus {
            is_error: StringNumber::from(0),
            error_description: String::new(),
        });

        assert_eq!(status.is_successful(), Some(true));
        assert!(status.status_description().contains("successful"));
    }

    #[test]
    fn test_transaction_status_with_execution_failure() {
        let tx_hash = "0x1234567890123456789012345678901234567890123456789012345678901234";
        let mut status = TransactionStatus::new(tx_hash);

        status.contract_execution = Some(ContractExecutionStatus {
            is_error: StringNumber::from(1),
            error_description: "Out of gas".to_string(),
        });

        assert_eq!(status.is_successful(), Some(false));
        assert!(status.status_description().contains("Out of gas"));
    }

    #[test]
    fn test_transaction_status_receipt_overrides_execution() {
        let tx_hash = "0x1234567890123456789012345678901234567890123456789012345678901234";
        let mut status = TransactionStatus::new(tx_hash);

        // Set execution as failed
        status.contract_execution = Some(ContractExecutionStatus {
            is_error: StringNumber::from(1),
            error_description: "Revert".to_string(),
        });

        // Set receipt as successful (should take precedence)
        status.receipt_status = Some(TransactionReceiptStatus {
            status: StringNumber::from(1),
        });

        // Receipt status should override execution status
        assert_eq!(status.is_successful(), Some(true));
        assert!(status.status_description().contains("successful"));
    }

    #[test]
    fn test_transaction_status_execution_fallback() {
        let tx_hash = "0x1234567890123456789012345678901234567890123456789012345678901234";
        let mut status = TransactionStatus::new(tx_hash);

        // Only set execution status (no receipt)
        status.contract_execution = Some(ContractExecutionStatus {
            is_error: StringNumber::from(1),
            error_description: "Execution reverted".to_string(),
        });

        // Should use execution status since no receipt available
        assert_eq!(status.is_successful(), Some(false));
        assert!(status.status_description().contains("Execution reverted"));
    }

    #[test]
    fn test_transaction_status_unknown() {
        let tx_hash = "0x1234567890123456789012345678901234567890123456789012345678901234";
        let status = TransactionStatus::new(tx_hash);

        // No status information available
        assert!(status.is_successful().is_none());
        assert_eq!(status.status_description(), "Status unknown");
    }
}

/// Test edge cases and error handling
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_batch_builder_large_number_of_transactions() {
        let client = TestUtils::create_test_client();
        let transactions = client.transactions();

        let mut builder = transactions.batch_status();

        // Add 100 transactions
        for i in 0..100 {
            let tx_hash = format!("0x{:064x}", i + 1);
            builder = builder.transaction(tx_hash);
        }

        assert_eq!(builder.get_tx_hashes().len(), 100);
    }

    #[test]
    fn test_duplicate_transaction_hashes() {
        let client = TestUtils::create_test_client();
        let transactions = client.transactions();

        let tx_hash = TestUtils::valid_tx_hash();
        let builder = transactions
            .batch_status()
            .transaction(tx_hash)
            .transaction(tx_hash)
            .transaction(tx_hash);

        // Should allow duplicates (API might handle deduplication)
        assert_eq!(builder.get_tx_hashes().len(), 3);
        for hash in builder.get_tx_hashes() {
            assert_eq!(hash.as_str(), tx_hash);
        }
    }

    #[test]
    fn test_status_builder_mode_switching() {
        let client = TestUtils::create_test_client();
        let transactions = client.transactions();

        let builder = transactions
            .batch_status()
            .transaction(TestUtils::valid_tx_hash())
            .execution_only()
            .receipt_only()
            .execution_only();

        // Should be in execution_only mode (last setting wins)
        assert!(builder.get_check_execution());
        assert!(!builder.get_check_receipt());
    }
}

/// Network-specific tests
mod network_tests {
    use super::*;
    use keion_etherscan::Network;

    #[test]
    fn test_transactions_endpoint_available_all_networks() {
        let networks = vec![
            Network::Mainnet,
            Network::Goerli,
            Network::Sepolia,
            Network::BinanceSmartChain,
            Network::Polygon,
            Network::Fantom,
            Network::Arbitrum,
            Network::Optimism,
        ];

        for network in networks {
            let client = TestUtils::create_test_client_for_network(network);
            let transactions = client.transactions();

            // Should be able to create builders on all networks
            let _builder = transactions
                .batch_status()
                .transaction(TestUtils::valid_tx_hash());
        }
    }
}

/// Test error handling scenarios
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_model_with_invalid_status_values() {
        // Test edge case status values
        let execution_status = ContractExecutionStatus {
            is_error: StringNumber::from(2), // Invalid value
            error_description: "Unknown error".to_string(),
        };

        // Should handle gracefully
        assert!(!execution_status.is_successful()); // Non-zero is failure
        assert!(execution_status.is_failed()); // Non-zero is failure

        let receipt_status = TransactionReceiptStatus {
            status: StringNumber::from(2), // Invalid value
        };

        assert!(!receipt_status.is_successful()); // Non-1 is failure
        assert!(receipt_status.is_failed()); // Non-1 is failure
    }

    #[test]
    fn test_transaction_status_with_zero_hash() {
        let zero_hash = "0x0000000000000000000000000000000000000000000000000000000000000000";
        let status = TransactionStatus::new(zero_hash);

        assert_eq!(status.tx_hash.as_str(), zero_hash);
        assert!(status.is_successful().is_none());
    }
}
