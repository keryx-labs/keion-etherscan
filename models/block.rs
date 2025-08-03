use crate::models::{BlockchainData, StringNumber};
use serde::{Deserialize, Serialize};

/// Placeholder for block-related models
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Block {
    /// TODO: Implement block model
    pub number: StringNumber,
}

impl BlockchainData for Block {
    fn block_number(&self) -> Option<u64> {
        Some(self.number.value())
    }

    fn timestamp(&self) -> Option<u64> {
        None
    }
}
