use crate::models::{Address, BigNumber, BlockchainData, StringNumber};
use serde::{Deserialize, Serialize};

/// ETH balance information for an account
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Balance {
    /// The account address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<Address>,

    /// Balance in wei (as string to handle large numbers)
    pub balance: BigNumber,
}

impl Balance {
    /// Get balance in wei as a string
    pub fn wei(&self) -> &str {
        self.balance.as_str()
    }

    /// Get balance in ETH (convenience method)
    /// Returns None if the balance is too large or invalid
    pub fn eth(&self) -> Option<f64> {
        self.balance.as_u128().map(|wei| wei as f64 / 1e18)
    }

    /// Get balance in gwei (convenience method)
    /// Returns None if the balance is too large or invalid
    pub fn gwei(&self) -> Option<f64> {
        self.balance.as_u128().map(|wei| wei as f64 / 1e9)
    }
}

impl BlockchainData for Balance {
    fn block_number(&self) -> Option<u64> {
        None // Balance doesn't have an associated block
    }

    fn timestamp(&self) -> Option<u64> {
        None // Balance doesn't have an associated timestamp
    }
}

/// ERC-20 token balance information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenBalance {
    /// Token contract address
    #[serde(rename = "TokenAddress")]
    pub contract_address: Address,

    /// Token name
    #[serde(rename = "TokenName")]
    pub name: String,

    /// Token symbol
    #[serde(rename = "TokenSymbol")]
    pub symbol: String,

    /// Token decimals
    #[serde(
        rename = "TokenDecimal",
        deserialize_with = "crate::models::deserialize_optional_string_number"
    )]
    pub decimals: Option<u64>,

    /// Token quantity (raw amount)
    #[serde(rename = "TokenQuantity")]
    pub quantity: BigNumber,
}

impl TokenBalance {
    /// Get the token quantity as a decimal value
    /// Returns None if decimals is not available or conversion fails
    pub fn decimal_quantity(&self) -> Option<f64> {
        let decimals = self.decimals?;
        let quantity = self.quantity.as_u128()?;
        Some(quantity as f64 / 10_f64.powi(decimals as i32))
    }

    /// Check if this is a zero balance
    pub fn is_zero(&self) -> bool {
        self.quantity.as_str() == "0"
    }
}

impl BlockchainData for TokenBalance {
    fn block_number(&self) -> Option<u64> {
        None
    }

    fn timestamp(&self) -> Option<u64> {
        None
    }
}

/// Account statistics and information
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountInfo {
    /// Account address
    pub address: Address,

    /// Current ETH balance
    pub balance: Balance,

    /// Total number of transactions (in + out)
    #[serde(rename = "txCount")]
    pub transaction_count: Option<StringNumber>,

    /// First transaction block number
    #[serde(rename = "firstTxBlock")]
    pub first_tx_block: Option<StringNumber>,

    /// Last transaction block number
    #[serde(rename = "lastTxBlock")]
    pub last_tx_block: Option<StringNumber>,
}

impl AccountInfo {
    /// Get transaction count as u64
    pub fn tx_count(&self) -> Option<u64> {
        self.transaction_count.as_ref().map(|c| c.value())
    }

    /// Get first transaction block number
    pub fn first_block(&self) -> Option<u64> {
        self.first_tx_block.as_ref().map(|b| b.value())
    }

    /// Get last transaction block number
    pub fn last_block(&self) -> Option<u64> {
        self.last_tx_block.as_ref().map(|b| b.value())
    }

    /// Check if this account has any transaction history
    pub fn has_transactions(&self) -> bool {
        self.tx_count().unwrap_or(0) > 0
    }
}

impl BlockchainData for AccountInfo {
    fn block_number(&self) -> Option<u64> {
        self.last_block()
    }

    fn timestamp(&self) -> Option<u64> {
        None
    }
}

/// Multi-balance response for batch balance queries
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiBalance {
    /// Account address
    pub account: Address,

    /// Balance in wei
    pub balance: BigNumber,
}

impl MultiBalance {
    /// Convert to a standard Balance struct
    pub fn to_balance(self) -> Balance {
        Balance {
            account: Some(self.account),
            balance: self.balance,
        }
    }

    /// Get balance in ETH
    pub fn eth(&self) -> Option<f64> {
        self.balance.as_u128().map(|wei| wei as f64 / 1e18)
    }
}

impl BlockchainData for MultiBalance {
    fn block_number(&self) -> Option<u64> {
        None
    }

    fn timestamp(&self) -> Option<u64> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_balance_eth_conversion() {
        let balance = Balance {
            account: Some(Address::new("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6")),
            balance: BigNumber::from("1000000000000000000".to_string()), // 1 ETH in wei
        };

        assert_eq!(balance.eth(), Some(1.0));
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
    }

    #[test]
    fn test_balance_deserialization() {
        let json = r#"{"balance": "123456789"}"#;
        let balance: Balance = serde_json::from_str(json).unwrap();
        assert_eq!(balance.balance.as_str(), "123456789");
    }

    #[test]
    fn test_multi_balance_conversion() {
        let multi = MultiBalance {
            account: Address::new("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6"),
            balance: BigNumber::from("1000000000000000000".to_string()),
        };

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
    }
}
