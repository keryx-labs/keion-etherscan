use keion_etherscan::{
    AccountInfo, Address, Balance, BigNumber, CodeFormat, ContractAbi, ContractCreation,
    ContractSource, HexNumber, InternalTransaction, LibraryLink, MultiBalance,
    OptimizationSettings, ProxyVerificationStatus, StringNumber, TokenBalance, TokenTransfer,
    Transaction, TxHash, VerificationRequest, VerificationStatus,
};
use serde_json;

#[test]
fn test_string_number_deserialization() {
    let json = r#""12345""#;
    let num: StringNumber = serde_json::from_str(json).unwrap();
    assert_eq!(num.value(), 12345);
    assert_eq!(num.to_string(), "12345");
}

#[test]
fn test_hex_number_deserialization() {
    let json = r#""0x1a2b""#;
    let num: HexNumber = serde_json::from_str(json).unwrap();
    assert_eq!(num.value(), 0x1a2b);
    assert_eq!(num.to_string(), "0x1a2b");

    // Test without 0x prefix
    let json2 = r#""1a2b""#;
    let num2: HexNumber = serde_json::from_str(json2).unwrap();
    assert_eq!(num2.value(), 0x1a2b);
}

#[test]
fn test_big_number() {
    let big = BigNumber::from("123456789012345678901234567890".to_string());
    assert_eq!(big.as_str(), "123456789012345678901234567890");
    assert!(big.as_u64().is_none()); // Too big for u64
    assert!(big.as_u128().is_some()); // Fits in u128
    assert_eq!(big.to_string(), "123456789012345678901234567890");

    let small = BigNumber::from("12345".to_string());
    assert_eq!(small.as_u64(), Some(12345));
    assert_eq!(small.as_u128(), Some(12345));
}

#[test]
fn test_address_normalization() {
    let addr = Address::new("0xABCDEF1234567890ABCDef1234567890abcdef12");
    assert_eq!(addr.as_str(), "0xabcdef1234567890abcdef1234567890abcdef12");
    assert_eq!(
        addr.to_string(),
        "0xabcdef1234567890abcdef1234567890abcdef12"
    );

    let addr_from_str: Address = "0xABCDEF1234567890ABCDef1234567890abcdef12".into();
    assert_eq!(
        addr_from_str.as_str(),
        "0xabcdef1234567890abcdef1234567890abcdef12"
    );
}

#[test]
fn test_zero_address() {
    let zero = Address::new("0x0000000000000000000000000000000000000000");
    assert!(zero.is_zero());

    let non_zero = Address::new("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6");
    assert!(!non_zero.is_zero());
}

#[test]
fn test_tx_hash() {
    let hash = TxHash::new("0xABCDEF1234567890ABCDef1234567890abcdef12345678901234567890abcdef");
    assert_eq!(
        hash.as_str(),
        "0xabcdef1234567890abcdef1234567890abcdef12345678901234567890abcdef"
    );
    assert_eq!(
        hash.to_string(),
        "0xabcdef1234567890abcdef1234567890abcdef12345678901234567890abcdef"
    );

    let hash_from_str: TxHash =
        "0xABCDEF1234567890ABCDef1234567890abcdef12345678901234567890abcdef".into();
    assert_eq!(
        hash_from_str.as_str(),
        "0xabcdef1234567890abcdef1234567890abcdef12345678901234567890abcdef"
    );
}

#[test]
fn test_balance_eth_conversion() {
    let balance = Balance {
        account: Some(Address::new("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6")),
        balance: BigNumber::from("1000000000000000000".to_string()), // 1 ETH in wei
    };

    assert_eq!(balance.wei(), "1000000000000000000");
    assert_eq!(balance.eth(), Some(1.0));
    assert_eq!(balance.gwei(), Some(1_000_000_000.0));
}

#[test]
fn test_token_balance_decimal_conversion() {
    let token_balance = TokenBalance {
        contract_address: Address::new("0xa0b86a33e6411b7a0a6acc95b0e8fd65b7b1b6c8"),
        name: "Test Token".to_string(),
        symbol: "TEST".to_string(),
        decimals: Some(18),
        quantity: BigNumber::from("1000000000000000000".to_string()), // 1 token with 18 decimals
    };

    assert_eq!(token_balance.decimal_quantity(), Some(1.0));
    assert!(!token_balance.is_zero());

    // Test with different decimals
    let token_6_decimals = TokenBalance {
        contract_address: Address::new("0xa0b86a33e6411b7a0a6acc95b0e8fd65b7b1b6c8"),
        name: "USDC".to_string(),
        symbol: "USDC".to_string(),
        decimals: Some(6),
        quantity: BigNumber::from("1000000".to_string()), // 1 USDC with 6 decimals
    };

    assert_eq!(token_6_decimals.decimal_quantity(), Some(1.0));
}

#[test]
fn test_token_balance_zero_check() {
    let zero_balance = TokenBalance {
        contract_address: Address::new("0xa0b86a33e6411b7a0a6acc95b0e8fd65b7b1b6c8"),
        name: "Test Token".to_string(),
        symbol: "TEST".to_string(),
        decimals: Some(18),
        quantity: BigNumber::from("0".to_string()),
    };

    assert!(zero_balance.is_zero());
    assert_eq!(zero_balance.decimal_quantity(), Some(0.0));
}

#[test]
fn test_multi_balance_conversion() {
    let multi = MultiBalance {
        account: Address::new("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6"),
        balance: BigNumber::from("1000000000000000000".to_string()),
    };

    assert_eq!(multi.eth(), Some(1.0));

    let balance = multi.to_balance();
    assert_eq!(
        balance.account.unwrap().as_str(),
        "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6"
    );
    assert_eq!(balance.balance.as_str(), "1000000000000000000");
}

#[test]
fn test_account_info_helpers() {
    let account_info = AccountInfo {
        address: Address::new("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6"),
        balance: Balance {
            account: None,
            balance: BigNumber::from("1000000000000000000".to_string()),
        },
        transaction_count: Some(StringNumber::from(42)),
        first_tx_block: Some(StringNumber::from(1000)),
        last_tx_block: Some(StringNumber::from(2000)),
    };

    assert_eq!(account_info.tx_count(), Some(42));
    assert_eq!(account_info.first_block(), Some(1000));
    assert_eq!(account_info.last_block(), Some(2000));
    assert!(account_info.has_transactions());

    // Test account with no transactions
    let empty_account = AccountInfo {
        address: Address::new("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a7"),
        balance: Balance {
            account: None,
            balance: BigNumber::from("0".to_string()),
        },
        transaction_count: None,
        first_tx_block: None,
        last_tx_block: None,
    };

    assert_eq!(empty_account.tx_count(), None);
    assert!(!empty_account.has_transactions());
}

#[test]
fn test_balance_deserialization() {
    let json = r#"{"balance": "123456789"}"#;
    let balance: Balance = serde_json::from_str(json).unwrap();
    assert_eq!(balance.balance.as_str(), "123456789");
    assert_eq!(balance.account, None);

    let json_with_account =
        r#"{"account": "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6", "balance": "123456789"}"#;
    let balance_with_account: Balance = serde_json::from_str(json_with_account).unwrap();
    assert_eq!(
        balance_with_account.account.unwrap().as_str(),
        "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6"
    );
}

// Contract model tests

#[test]
fn test_contract_abi_deserialization() {
    let json = r#"{"ABI": "[{\"type\":\"function\",\"name\":\"getValue\",\"inputs\":[],\"outputs\":[{\"type\":\"uint256\"}]}]"}"#;
    let abi: ContractAbi = serde_json::from_str(json).unwrap();

    assert!(!abi.is_empty());
    assert_eq!(abi.abi, "[{\"type\":\"function\",\"name\":\"getValue\",\"inputs\":[],\"outputs\":[{\"type\":\"uint256\"}]}]");

    // Test ABI parsing
    let parsed = abi.parse_abi().unwrap();
    assert!(parsed.is_array());
}

#[test]
fn test_contract_abi_empty_check() {
    let empty_abi = ContractAbi {
        abi: "".to_string(),
    };
    assert!(empty_abi.is_empty());

    let not_verified_abi = ContractAbi {
        abi: "Contract source code not verified".to_string(),
    };
    assert!(not_verified_abi.is_empty());

    let valid_abi = ContractAbi {
        abi: "[]".to_string(),
    };
    assert!(!valid_abi.is_empty());
}

#[test]
fn test_contract_source_deserialization() {
    let json = r#"{
        "SourceCode": "pragma solidity ^0.8.0; contract Test {}",
        "ABI": "[{\"type\":\"constructor\"}]",
        "ContractName": "Test",
        "CompilerVersion": "v0.8.24+commit.e11b9ed9",
        "OptimizationUsed": "1",
        "Runs": "200",
        "ConstructorArguments": "0x000000000000000000000000000000000000000000000000000000000000007b",
        "EVMVersion": "default",
        "Library": "",
        "LicenseType": "MIT",
        "Proxy": "0",
        "Implementation": "",
        "SwarmSource": ""
    }"#;

    let source: ContractSource = serde_json::from_str(json).unwrap();

    assert_eq!(source.contract_name, "Test");
    assert_eq!(source.compiler_version, "v0.8.24+commit.e11b9ed9");
    assert!(source.is_optimized());
    assert_eq!(source.optimization_runs(), 200);
    assert!(!source.is_proxy());
    assert!(source.is_verified());

    // Test ABI parsing
    let parsed_abi = source.parse_abi().unwrap();
    assert!(parsed_abi.is_array());
}

#[test]
fn test_contract_source_helpers() {
    let optimized_source = ContractSource {
        source_code: "pragma solidity ^0.8.0; contract Test {}".to_string(),
        abi: "[]".to_string(),
        contract_name: "Test".to_string(),
        compiler_version: "v0.8.24".to_string(),
        optimization_used: StringNumber::from(1),
        runs: StringNumber::from(200),
        constructor_arguments: "".to_string(),
        evm_version: "default".to_string(),
        library: "".to_string(),
        license_type: "MIT".to_string(),
        proxy: StringNumber::from(0),
        implementation: "".to_string(),
        swarm_source: "".to_string(),
    };

    assert!(optimized_source.is_optimized());
    assert!(!optimized_source.is_proxy());
    assert_eq!(optimized_source.optimization_runs(), 200);
    assert!(optimized_source.is_verified());

    let proxy_source = ContractSource {
        source_code: "".to_string(),
        abi: "".to_string(),
        contract_name: "".to_string(),
        compiler_version: "".to_string(),
        optimization_used: StringNumber::from(0),
        runs: StringNumber::from(0),
        constructor_arguments: "".to_string(),
        evm_version: "".to_string(),
        library: "".to_string(),
        license_type: "".to_string(),
        proxy: StringNumber::from(1),
        implementation: "0x1234567890123456789012345678901234567890".to_string(),
        swarm_source: "".to_string(),
    };

    assert!(!proxy_source.is_optimized());
    assert!(proxy_source.is_proxy());
    assert_eq!(proxy_source.optimization_runs(), 0);
    assert!(!proxy_source.is_verified());
}

#[test]
fn test_contract_source_unverified() {
    let unverified_source = ContractSource {
        source_code: "Contract source code not verified".to_string(),
        abi: "Contract source code not verified".to_string(),
        contract_name: "".to_string(),
        compiler_version: "".to_string(),
        optimization_used: StringNumber::from(0),
        runs: StringNumber::from(0),
        constructor_arguments: "".to_string(),
        evm_version: "".to_string(),
        library: "".to_string(),
        license_type: "".to_string(),
        proxy: StringNumber::from(0),
        implementation: "".to_string(),
        swarm_source: "".to_string(),
    };

    assert!(!unverified_source.is_verified());
    assert!(!unverified_source.is_optimized());
    assert!(!unverified_source.is_proxy());
}

#[test]
fn test_contract_creation_deserialization() {
    let json = r#"{
        "contractAddress": "0xa0b86a33e6411b7a0a6acc95b0e8fd65b7b1b6c8",
        "contractCreator": "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6",
        "txHash": "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef12"
    }"#;

    let creation: ContractCreation = serde_json::from_str(json).unwrap();

    assert_eq!(
        creation.contract_address.as_str(),
        "0xa0b86a33e6411b7a0a6acc95b0e8fd65b7b1b6c8"
    );
    assert_eq!(
        creation.contract_creator.as_str(),
        "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6"
    );
    assert_eq!(
        creation.tx_hash.as_str(),
        "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef12"
    );
}

#[test]
fn test_verification_status_helpers() {
    let verified_status = VerificationStatus {
        status: "Pass - Verified".to_string(),
    };
    assert!(verified_status.is_verified());
    assert!(!verified_status.is_failed());
    assert!(!verified_status.is_pending());

    let failed_status = VerificationStatus {
        status: "Fail - Unable to verify".to_string(),
    };
    assert!(!failed_status.is_verified());
    assert!(failed_status.is_failed());
    assert!(!failed_status.is_pending());

    let pending_status = VerificationStatus {
        status: "Pending in queue".to_string(),
    };
    assert!(!pending_status.is_verified());
    assert!(!pending_status.is_failed());
    assert!(pending_status.is_pending());
}

#[test]
fn test_proxy_verification_status() {
    let successful_proxy = ProxyVerificationStatus {
        result: "The proxy's implementation contract is successfully updated".to_string(),
    };
    assert!(successful_proxy.is_verified());

    let failed_proxy = ProxyVerificationStatus {
        result: "Unable to update proxy implementation".to_string(),
    };
    assert!(!failed_proxy.is_verified());
}

#[test]
fn test_verification_request_deserialization() {
    let json = r#"{"guid": "ezq878u486pzijgvynpjq"}"#;
    let request: VerificationRequest = serde_json::from_str(json).unwrap();

    assert_eq!(request.guid, "ezq878u486pzijgvynpjq");
}

#[test]
fn test_code_format_string_representations() {
    assert_eq!(
        CodeFormat::SoliditySingleFile.as_str(),
        "solidity-single-file"
    );
    assert_eq!(
        CodeFormat::SolidityStandardJsonInput.as_str(),
        "solidity-standard-json-input"
    );
    assert_eq!(CodeFormat::VyperJson.as_str(), "vyper-json");
}

#[test]
fn test_optimization_settings() {
    let enabled = OptimizationSettings::enabled(200);
    assert!(enabled.enabled);
    assert_eq!(enabled.runs, 200);

    let disabled = OptimizationSettings::disabled();
    assert!(!disabled.enabled);
    assert_eq!(disabled.runs, 0);

    // Test with high optimization runs
    let high_opt = OptimizationSettings::enabled(10000);
    assert!(high_opt.enabled);
    assert_eq!(high_opt.runs, 10000);
}

#[test]
fn test_library_link() {
    let link = LibraryLink::new("SafeMath", "0x1234567890123456789012345678901234567890");
    assert_eq!(link.name, "SafeMath");
    assert_eq!(link.address, "0x1234567890123456789012345678901234567890");

    // Test with different string types
    let link_string = LibraryLink::new(
        String::from("StringUtils"),
        String::from("0x0987654321098765432109876543210987654321"),
    );
    assert_eq!(link_string.name, "StringUtils");
    assert_eq!(
        link_string.address,
        "0x0987654321098765432109876543210987654321"
    );

    // Test with &str
    let link_str = LibraryLink::new("Address", "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd");
    assert_eq!(link_str.name, "Address");
    assert_eq!(
        link_str.address,
        "0xabcdefabcdefabcdefabcdefabcdefabcdefabcd"
    );
}

// Test deserialization of real-world like data
#[test]
fn test_contract_models_realistic_data() {
    // Test with realistic USDT contract data
    let usdt_abi_json = r#"{"ABI": "[{\"constant\":true,\"inputs\":[],\"name\":\"name\",\"outputs\":[{\"name\":\"\",\"type\":\"string\"}],\"payable\":false,\"stateMutability\":\"view\",\"type\":\"function\"},{\"constant\":false,\"inputs\":[{\"name\":\"_spender\",\"type\":\"address\"},{\"name\":\"_value\",\"type\":\"uint256\"}],\"name\":\"approve\",\"outputs\":[],\"payable\":false,\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]"}"#;

    let abi: ContractAbi = serde_json::from_str(usdt_abi_json).unwrap();
    assert!(!abi.is_empty());

    let parsed_abi = abi.parse_abi().unwrap();
    assert!(parsed_abi.is_array());
    assert_eq!(parsed_abi.as_array().unwrap().len(), 2);

    // Test with realistic contract source
    let source_json = r#"{
        "SourceCode": "pragma solidity 0.4.17;\n\ncontract TetherToken {\n    string public name;\n    function transfer(address _to, uint _value) public;\n}",
        "ABI": "[{\"constant\":false,\"inputs\":[{\"name\":\"_to\",\"type\":\"address\"},{\"name\":\"_value\",\"type\":\"uint256\"}],\"name\":\"transfer\",\"outputs\":[],\"payable\":false,\"stateMutability\":\"nonpayable\",\"type\":\"function\"}]",
        "ContractName": "TetherToken",
        "CompilerVersion": "v0.4.17+commit.bdeb9e52",
        "OptimizationUsed": "1",
        "Runs": "200",
        "ConstructorArguments": "0000000000000000000000000000000000000000000000000000000000000064",
        "EVMVersion": "default",
        "Library": "",
        "LicenseType": "",
        "Proxy": "0",
        "Implementation": "",
        "SwarmSource": "bzzr://abc123"
    }"#;

    let source: ContractSource = serde_json::from_str(source_json).unwrap();
    assert_eq!(source.contract_name, "TetherToken");
    assert_eq!(source.compiler_version, "v0.4.17+commit.bdeb9e52");
    assert!(source.is_optimized());
    assert!(source.is_verified());
    assert!(!source.is_proxy());
    assert_eq!(source.optimization_runs(), 200);

    // Test with proxy contract data
    let proxy_json = r#"{
        "SourceCode": "",
        "ABI": "Contract source code not verified",
        "ContractName": "",
        "CompilerVersion": "",
        "OptimizationUsed": "0",
        "Runs": "0",
        "ConstructorArguments": "",
        "EVMVersion": "",
        "Library": "",
        "LicenseType": "",
        "Proxy": "1",
        "Implementation": "0x1234567890123456789012345678901234567890",
        "SwarmSource": ""
    }"#;

    let proxy_source: ContractSource = serde_json::from_str(proxy_json).unwrap();
    assert!(proxy_source.is_proxy());
    assert!(!proxy_source.is_verified());
    assert!(!proxy_source.is_optimized());
    assert_eq!(
        proxy_source.implementation,
        "0x1234567890123456789012345678901234567890"
    );
}

#[test]
fn test_contract_verification_edge_cases() {
    // Test with empty strings and edge cases
    let minimal_verification = VerificationStatus {
        status: "".to_string(),
    };
    assert!(!minimal_verification.is_verified());
    assert!(!minimal_verification.is_failed());
    assert!(!minimal_verification.is_pending());

    // Test with unusual status messages
    let custom_pass_status = VerificationStatus {
        status: "Pass - Already verified".to_string(),
    };
    assert!(custom_pass_status.is_verified());

    let custom_fail_status = VerificationStatus {
        status: "Fail - Compiler version mismatch".to_string(),
    };
    assert!(custom_fail_status.is_failed());

    let custom_pending_status = VerificationStatus {
        status: "Already Pending in queue".to_string(),
    };
    assert!(custom_pending_status.is_pending());
}

#[test]
fn test_library_link_edge_cases() {
    // Test with very long names
    let long_name_link = LibraryLink::new(
        "VeryLongLibraryNameThatExceedsNormalExpectationsButShouldStillWork",
        "0x1234567890123456789012345678901234567890",
    );
    assert_eq!(
        long_name_link.name,
        "VeryLongLibraryNameThatExceedsNormalExpectationsButShouldStillWork"
    );

    // Test with empty strings (although this wouldn't be practical)
    let empty_link = LibraryLink::new("", "");
    assert_eq!(empty_link.name, "");
    assert_eq!(empty_link.address, "");

    // Test with special characters in name
    let special_char_link = LibraryLink::new("SafeMath_v2.0", "0xabcdef123456");
    assert_eq!(special_char_link.name, "SafeMath_v2.0");
}

#[test]
fn test_code_format_exhaustive() {
    // Test all code format variants
    let formats = vec![
        CodeFormat::SoliditySingleFile,
        CodeFormat::SolidityStandardJsonInput,
        CodeFormat::VyperJson,
    ];

    let expected_strings = vec![
        "solidity-single-file",
        "solidity-standard-json-input",
        "vyper-json",
    ];

    for (format, expected) in formats.iter().zip(expected_strings.iter()) {
        assert_eq!(format.as_str(), *expected);
    }
}

#[test]
fn test_optimization_settings_edge_cases() {
    // Test with maximum runs
    let max_opt = OptimizationSettings::enabled(u32::MAX);
    assert!(max_opt.enabled);
    assert_eq!(max_opt.runs, u32::MAX);

    // Test with zero runs but enabled (unusual but valid)
    let zero_runs_enabled = OptimizationSettings {
        enabled: true,
        runs: 0,
    };
    assert!(zero_runs_enabled.enabled);
    assert_eq!(zero_runs_enabled.runs, 0);

    // Test with runs but disabled (contradictory but allowed by struct)
    let disabled_with_runs = OptimizationSettings {
        enabled: false,
        runs: 200,
    };
    assert!(!disabled_with_runs.enabled);
    assert_eq!(disabled_with_runs.runs, 200);
}
