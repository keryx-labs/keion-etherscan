use serde::{Deserialize, Serialize};

/// Placeholder for gas-related models
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GasPrice {
    /// TODO: Implement gas price model
    pub price: String,
}