use crate::models::{Address, BigNumber, BlockchainData, StringNumber};
use serde::{Deserialize, Serialize};

/// Beacon chain withdrawal event for a validator
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BeaconWithdrawal {
    /// Withdrawal index (unique identifier)
    #[serde(rename = "withdrawalIndex")]
    pub withdrawal_index: StringNumber,

    /// Validator index
    #[serde(rename = "validatorIndex")]
    pub validator_index: StringNumber,

    /// Withdrawal address (recipient)
    pub address: Address,

    /// Amount withdrawn in Gwei
    pub amount: BigNumber,

    /// Block number when the withdrawal occurred
    #[serde(rename = "blockNumber")]
    pub block_number: StringNumber,

    /// Timestamp of the withdrawal
    #[serde(rename = "timestamp")]
    pub timestamp: StringNumber,
}

impl BeaconWithdrawal {
    /// Get withdrawal index as u64
    pub fn index(&self) -> u64 {
        self.withdrawal_index.value()
    }

    /// Get validator index as u64
    pub fn validator(&self) -> u64 {
        self.validator_index.value()
    }

    /// Get block number as u64
    pub fn block(&self) -> u64 {
        self.block_number.value()
    }

    /// Get timestamp as u64
    pub fn timestamp_value(&self) -> u64 {
        self.timestamp.value()
    }

    /// Get withdrawal amount in Gwei as string
    pub fn amount_gwei(&self) -> &str {
        self.amount.as_str()
    }

    /// Get withdrawal amount in ETH
    pub fn amount_eth(&self) -> Option<f64> {
        // Convert from Gwei to ETH (1 ETH = 1e9 Gwei)
        self.amount.as_u128().map(|gwei| gwei as f64 / 1e9)
    }

    /// Get withdrawal amount in Wei
    pub fn amount_wei(&self) -> Option<u128> {
        // Convert from Gwei to Wei (1 Gwei = 1e9 Wei)
        self.amount.as_u128().map(|gwei| gwei * 1_000_000_000)
    }
}

impl BlockchainData for BeaconWithdrawal {
    fn block_number(&self) -> Option<u64> {
        Some(self.block())
    }

    fn timestamp(&self) -> Option<u64> {
        Some(self.timestamp_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_beacon_withdrawal_deserialization() {
        let json = r#"{
            "withdrawalIndex": "1234567",
            "validatorIndex": "123456",
            "address": "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6",
            "amount": "32000000000",
            "blockNumber": "17000000",
            "timestamp": "1681228800"
        }"#;

        let withdrawal: BeaconWithdrawal = serde_json::from_str(json).unwrap();
        assert_eq!(withdrawal.index(), 1234567);
        assert_eq!(withdrawal.validator(), 123456);
        assert_eq!(withdrawal.block(), 17000000);
        assert_eq!(withdrawal.timestamp_value(), 1681228800);
        assert_eq!(withdrawal.amount_gwei(), "32000000000");
    }

    #[test]
    fn test_beacon_withdrawal_amount_conversions() {
        let withdrawal = BeaconWithdrawal {
            withdrawal_index: StringNumber::from(1000),
            validator_index: StringNumber::from(500),
            address: Address::new("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6"),
            amount: BigNumber::from("32000000000".to_string()), // 32 ETH in Gwei
            block_number: StringNumber::from(17000000),
            timestamp: StringNumber::from(1681228800),
        };

        // Test Gwei amount
        assert_eq!(withdrawal.amount_gwei(), "32000000000");

        // Test ETH conversion (32 billion Gwei = 32 ETH)
        assert_eq!(withdrawal.amount_eth(), Some(32.0));

        // Test Wei conversion (32 billion Gwei = 32 * 10^18 Wei)
        assert_eq!(withdrawal.amount_wei(), Some(32_000_000_000_000_000_000));
    }

    #[test]
    fn test_partial_withdrawal() {
        // Test a partial withdrawal (rewards only, not full validator balance)
        let withdrawal = BeaconWithdrawal {
            withdrawal_index: StringNumber::from(2000),
            validator_index: StringNumber::from(1000),
            address: Address::new("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6"),
            amount: BigNumber::from("1234567890".to_string()), // ~1.23 ETH in Gwei
            block_number: StringNumber::from(17500000),
            timestamp: StringNumber::from(1690000000),
        };

        // Should be approximately 1.23456789 ETH
        let eth_amount = withdrawal.amount_eth().unwrap();
        assert!((eth_amount - 1.23456789).abs() < 0.000000001);
    }
}
