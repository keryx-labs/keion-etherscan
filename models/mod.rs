//! Data models for Etherscan API responses
//!
//! This module contains strongly-typed representations of data returned
//! by the Etherscan API endpoints.

mod account;
mod transaction;
mod contract;
mod block;
mod token;
mod gas;

pub use account::*;
pub use transaction::*;
pub use contract::*;
pub use block::*;
pub use token::*;
pub use gas::*;

use serde::{Deserialize, Serialize};
use std::fmt;

/// Common trait for all models that represent blockchain data
pub trait BlockchainData {
    /// Get the block number associated with this data, if applicable
    fn block_number(&self) -> Option<u64>;

    /// Get the timestamp associated with this data, if applicable
    fn timestamp(&self) -> Option<u64>;
}

/// Helper type for handling string numbers from API
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct StringNumber(#[serde(deserialize_with = "deserialize_string_number")] pub u64);

impl StringNumber {
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl From<u64> for StringNumber {
    fn from(value: u64) -> Self {
        StringNumber(value)
    }
}

impl fmt::Display for StringNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Helper type for handling hex string numbers from API
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct HexNumber(#[serde(deserialize_with = "deserialize_hex_number")] pub u64);

impl HexNumber {
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl From<u64> for HexNumber {
    fn from(value: u64) -> Self {
        HexNumber(value)
    }
}

impl fmt::Display for HexNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:x}", self.0)
    }
}

/// Helper type for handling large number strings (like wei amounts)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BigNumber(#[serde(deserialize_with = "deserialize_big_number")] pub String);

impl BigNumber {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Parse as u64 if the number fits
    pub fn as_u64(&self) -> Option<u64> {
        self.0.parse().ok()
    }

    /// Parse as u128 if the number fits
    pub fn as_u128(&self) -> Option<u128> {
        self.0.parse().ok()
    }
}

impl From<String> for BigNumber {
    fn from(value: String) -> Self {
        BigNumber(value)
    }
}

impl fmt::Display for BigNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Helper type for Ethereum addresses
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Address(pub String);

impl Address {
    pub fn new<S: Into<String>>(address: S) -> Self {
        Address(address.into().to_lowercase())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Check if this is the zero address
    pub fn is_zero(&self) -> bool {
        self.0 == "0x0000000000000000000000000000000000000000"
    }
}

impl From<String> for Address {
    fn from(value: String) -> Self {
        Address::new(value)
    }
}

impl From<&str> for Address {
    fn from(value: &str) -> Self {
        Address::new(value)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Helper type for transaction hashes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TxHash(pub String);

impl TxHash {
    pub fn new<S: Into<String>>(hash: S) -> Self {
        TxHash(hash.into().to_lowercase())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for TxHash {
    fn from(value: String) -> Self {
        TxHash::new(value)
    }
}

impl From<&str> for TxHash {
    fn from(value: &str) -> Self {
        TxHash::new(value)
    }
}

impl fmt::Display for TxHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Custom deserializers for handling Etherscan's string formats
fn deserialize_string_number<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}

fn deserialize_hex_number<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let s = s.strip_prefix("0x").unwrap_or(&s);
    u64::from_str_radix(s, 16).map_err(serde::de::Error::custom)
}

fn deserialize_big_number<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    String::deserialize(deserializer)
}

/// Helper for optional string number deserialization
pub fn deserialize_optional_string_number<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) if !s.is_empty() => s.parse().map(Some).map_err(serde::de::Error::custom),
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_string_number_deserialization() {
        let json = r#""12345""#;
        let num: StringNumber = serde_json::from_str(json).unwrap();
        assert_eq!(num.value(), 12345);
    }

    #[test]
    fn test_hex_number_deserialization() {
        let json = r#""0x1a2b""#;
        let num: HexNumber = serde_json::from_str(json).unwrap();
        assert_eq!(num.value(), 0x1a2b);
    }

    #[test]
    fn test_address_normalization() {
        let addr = Address::new("0xABCDEF1234567890ABCDef1234567890abcdef12");
        assert_eq!(addr.as_str(), "0xabcdef1234567890abcdef1234567890abcdef12");
    }

    #[test]
    fn test_zero_address() {
        let zero = Address::new("0x0000000000000000000000000000000000000000");
        assert!(zero.is_zero());

        let non_zero = Address::new("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6");
        assert!(!non_zero.is_zero());
    }

    #[test]
    fn test_big_number() {
        let big = BigNumber::from("123456789012345678901234567890".to_string());
        assert!(big.as_u64().is_none()); // Too big for u64
        assert!(big.as_u128().is_some()); // Fits in u128
    }
}