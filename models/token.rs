use serde::{Deserialize, Serialize};

/// Placeholder for token-related models
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Token {
    /// TODO: Implement token model
    pub symbol: String,
}