use crate::models::{BlockchainData, StringNumber};
use serde::{Deserialize, Serialize};

/// Block validated by a validator address
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatedBlock {
    /// Block number
    #[serde(rename = "blockNumber")]
    pub block_number: StringNumber,

    /// Timestamp when the block was validated
    #[serde(rename = "timeStamp")]
    pub timestamp: StringNumber,

    /// Block reward received by the validator (in wei)
    #[serde(rename = "blockReward")]
    pub block_reward: StringNumber,
}

impl ValidatedBlock {
    /// Get block number as u64
    pub fn block(&self) -> u64 {
        self.block_number.value()
    }

    /// Get timestamp as u64
    pub fn timestamp_value(&self) -> u64 {
        self.timestamp.value()
    }

    /// Get block reward in ETH
    pub fn reward_eth(&self) -> Option<f64> {
        let reward_wei = self.block_reward.value() as f64;
        Some(reward_wei / 1e18)
    }

    /// Get block reward in wei as u64
    pub fn reward_wei(&self) -> u64 {
        self.block_reward.value()
    }
}

impl BlockchainData for ValidatedBlock {
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
    fn test_validated_block_deserialization() {
        let json = r#"{
            "blockNumber": "12345678",
            "timeStamp": "1234567890",
            "blockReward": "2000000000000000000"
        }"#;

        let block: ValidatedBlock = serde_json::from_str(json).unwrap();
        assert_eq!(block.block(), 12345678);
        assert_eq!(block.timestamp_value(), 1234567890);
        assert_eq!(block.reward_eth(), Some(2.0));
    }

    #[test]
    fn test_validated_block_helpers() {
        let block = ValidatedBlock {
            block_number: StringNumber::from(15000000),
            timestamp: StringNumber::from(1659312000),
            block_reward: StringNumber::from(2500000000000000000u64), // 2.5 ETH
        };

        assert_eq!(block.block(), 15000000);
        assert_eq!(block.timestamp_value(), 1659312000);
        assert_eq!(block.reward_eth(), Some(2.5));
        assert_eq!(block.reward_wei(), 2500000000000000000);
    }
}
