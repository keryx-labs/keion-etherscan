use keion_etherscan::{
    Address, TxHash, BigNumber, StringNumber, HexNumber,
    Balance, TokenBalance, MultiBalance, AccountInfo,
    Transaction, TokenTransfer, InternalTransaction,
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
    assert_eq!(addr.to_string(), "0xabcdef1234567890abcdef1234567890abcdef12");

    let addr_from_str: Address = "0xABCDEF1234567890ABCDef1234567890abcdef12".into();
    assert_eq!(addr_from_str.as_str(), "0xabcdef1234567890abcdef1234567890abcdef12");
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
    assert_eq!(hash.as_str(), "0xabcdef1234567890abcdef1234567890abcdef12345678901234567890abcdef");
    assert_eq!(hash.to_string(), "0xabcdef1234567890abcdef1234567890abcdef12345678901234567890abcdef");

    let hash_from_str: TxHash = "0xABCDEF1234567890ABCDef1234567890abcdef12345678901234567890abcdef".into();
    assert_eq!(hash_from_str.as_str(), "0xabcdef1234567890abcdef1234567890abcdef12345678901234567890abcdef");
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
    assert_eq!(balance.account.unwrap().as_str(), "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6");
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

    let json_with_account = r#"{"account": "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6", "balance": "123456789"}"#;
    let balance_with_account: Balance = serde_json::from_str(json_with_account).unwrap();
    assert_eq!(balance_with_account.account.unwrap().as_str(), "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6");
}