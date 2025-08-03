use keion_etherscan::{EtherscanClient, Network};
use serde_json::json;
use std::collections::HashMap;

/// Test utilities and common functions
pub struct TestUtils;

impl TestUtils {
    /// Create a test client with a dummy API key
    pub fn create_test_client() -> EtherscanClient {
        EtherscanClient::builder()
            .api_key("test-api-key-1234567890abcdef")
            .network(Network::Mainnet)
            .build()
            .unwrap()
    }

    /// Create a test client for a specific network
    pub fn create_test_client_for_network(network: Network) -> EtherscanClient {
        EtherscanClient::builder()
            .api_key("test-api-key-1234567890abcdef")
            .network(network)
            .build()
            .unwrap()
    }

    /// Valid test addresses for different scenarios
    pub fn valid_address() -> &'static str {
        "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6"
    }

    pub fn valid_address_mixed_case() -> &'static str {
        "0x742D35Cc6634C0532925a3B8d19389c4D5e1e4a6"
    }

    pub fn validator_address() -> &'static str {
        "0x1234567890123456789012345678901234567890"
    }

    pub fn contract_address() -> &'static str {
        "0xa0b86a33e6411b7a0a6acc95b0e8fd65b7b1b6c8"
    }

    /// Invalid test addresses for error testing
    pub fn invalid_address_too_short() -> &'static str {
        "0x123"
    }

    pub fn invalid_address_no_prefix() -> &'static str {
        "742d35cc6634c0532925a3b8d19389c4d5e1e4a6"
    }

    pub fn invalid_address_wrong_length() -> &'static str {
        "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a61"
    }

    pub fn invalid_address_non_hex() -> &'static str {
        "0x742d35cc6634c0532925a3b8d19389c4d5e1e4zz"
    }

    /// Valid transaction hashes
    pub fn valid_tx_hash() -> &'static str {
        "0x1234567890123456789012345678901234567890123456789012345678901234"
    }

    /// Invalid transaction hashes for error testing
    pub fn invalid_tx_hash_too_short() -> &'static str {
        "0x123456"
    }

    pub fn invalid_tx_hash_no_prefix() -> &'static str {
        "1234567890123456789012345678901234567890123456789012345678901234"
    }

    pub fn invalid_tx_hash_non_hex() -> &'static str {
        "0x123456789012345678901234567890123456789012345678901234567890123z"
    }
}

/// Mock response utilities for testing
pub struct MockResponses;

impl MockResponses {
    /// Mock successful balance response
    pub fn balance_response() -> serde_json::Value {
        json!({
            "status": "1",
            "message": "OK",
            "result": "1000000000000000000"
        })
    }

    /// Mock successful multi-balance response
    pub fn multi_balance_response() -> serde_json::Value {
        json!({
            "status": "1",
            "message": "OK",
            "result": [
                {
                    "account": "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6",
                    "balance": "1000000000000000000"
                },
                {
                    "account": "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a7",
                    "balance": "2000000000000000000"
                }
            ]
        })
    }

    /// Mock successful transactions response
    pub fn transactions_response() -> serde_json::Value {
        json!({
            "status": "1",
            "message": "OK",
            "result": [
                {
                    "blockNumber": "12345678",
                    "blockHash": "0xabcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
                    "transactionIndex": "1",
                    "hash": "0x1234567890123456789012345678901234567890123456789012345678901234",
                    "nonce": "42",
                    "from": "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6",
                    "to": "0xa0b86a33e6411b7a0a6acc95b0e8fd65b7b1b6c8",
                    "value": "1000000000000000000",
                    "gas": "21000",
                    "gasPrice": "20000000000",
                    "gasUsed": "21000",
                    "cumulativeGasUsed": "21000",
                    "input": "0x",
                    "timeStamp": "1234567890",
                    "methodId": null,
                    "functionName": null,
                    "txreceipt_status": "1",
                    "confirmations": "100",
                    "isError": "0"
                }
            ]
        })
    }

    /// Mock successful internal transactions response
    pub fn internal_transactions_response() -> serde_json::Value {
        json!({
            "status": "1",
            "message": "OK",
            "result": [
                {
                    "blockNumber": "12345678",
                    "hash": "0x1234567890123456789012345678901234567890123456789012345678901234",
                    "from": "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6",
                    "to": "0xa0b86a33e6411b7a0a6acc95b0e8fd65b7b1b6c8",
                    "value": "500000000000000000",
                    "contractAddress": null,
                    "input": "0x",
                    "type": "call",
                    "gas": "21000",
                    "gasUsed": "21000",
                    "traceId": "0",
                    "isError": "0",
                    "errCode": null,
                    "timeStamp": "1234567890"
                }
            ]
        })
    }

    /// Mock successful validated blocks response
    pub fn validated_blocks_response() -> serde_json::Value {
        json!({
            "status": "1",
            "message": "OK",
            "result": [
                {
                    "blockNumber": "15000000",
                    "timeStamp": "1659312000",
                    "blockReward": "2000000000000000000"
                },
                {
                    "blockNumber": "15000001",
                    "timeStamp": "1659312012",
                    "blockReward": "2100000000000000000"
                }
            ]
        })
    }

    /// Mock successful beacon withdrawals response
    pub fn beacon_withdrawals_response() -> serde_json::Value {
        json!({
            "status": "1",
            "message": "OK",
            "result": [
                {
                    "withdrawalIndex": "1234567",
                    "validatorIndex": "123456",
                    "address": "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6",
                    "amount": "32000000000",
                    "blockNumber": "17000000",
                    "timestamp": "1681228800"
                }
            ]
        })
    }

    /// Mock API error response
    pub fn api_error_response() -> serde_json::Value {
        json!({
            "status": "0",
            "message": "NOTOK",
            "result": "Invalid address format"
        })
    }

    /// Mock rate limit error response
    pub fn rate_limit_error_response() -> serde_json::Value {
        json!({
            "status": "0",
            "message": "NOTOK",
            "result": "Max rate limit reached"
        })
    }

    /// Mock empty result response
    pub fn empty_response() -> serde_json::Value {
        json!({
            "status": "1",
            "message": "No transactions found",
            "result": []
        })
    }
}

/// Constants for testing
pub struct TestConstants;

impl TestConstants {
    pub const MAINNET_BLOCK: u64 = 18_000_000;
    pub const GOERLI_BLOCK: u64 = 9_000_000;
    pub const RECENT_BLOCK: u64 = 19_000_000;
    pub const OLD_BLOCK: u64 = 1_000_000;
    
    pub const STANDARD_PAGE_SIZE: u32 = 100;
    pub const MAX_PAGE_SIZE: u32 = 10_000;
    pub const MAX_MULTI_ADDRESS_COUNT: usize = 20;
}