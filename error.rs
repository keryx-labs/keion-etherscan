use std::fmt;

/// Result type alias for keion-etherscan operations
pub type Result<T> = std::result::Result<T, EtherscanError>;

/// Comprehensive error types for the Etherscan client
#[derive(Debug, Clone)]
pub enum EtherscanError {
    /// Missing API key during client construction
    MissingApiKey,

    /// Invalid URL configuration
    InvalidUrl(String),

    /// HTTP client construction failed
    HttpClient(String),

    /// Network request failed
    Request(String),

    /// HTTP error response
    Http {
        status: u16,
        message: String,
    },

    /// API returned an error status
    Api {
        message: String,
        result: Option<String>,
    },

    /// Failed to parse response
    Response(String),

    /// JSON parsing error
    Parse(String),

    /// Invalid address format
    InvalidAddress(String),

    /// Invalid transaction hash format
    InvalidTxHash(String),

    /// Invalid block number or tag
    InvalidBlock(String),

    /// Rate limit exceeded
    RateLimit {
        retry_after: Option<u64>,
        message: String,
    },

    /// Timeout occurred
    Timeout(String),

    /// Invalid parameters provided
    InvalidParams(String),

    /// Feature not supported on this network
    UnsupportedNetwork {
        network: String,
        feature: String,
    },

    /// Generic internal error
    Internal(String),
}

impl EtherscanError {
    /// Create a new API error
    pub fn api<S: Into<String>>(message: S) -> Self {
        EtherscanError::Api {
            message: message.into(),
            result: None,
        }
    }

    /// Create a new API error with result details
    pub fn api_with_result<S: Into<String>, R: Into<String>>(message: S, result: R) -> Self {
        EtherscanError::Api {
            message: message.into(),
            result: Some(result.into()),
        }
    }

    /// Create a new rate limit error
    pub fn rate_limit<S: Into<String>>(message: S, retry_after: Option<u64>) -> Self {
        EtherscanError::RateLimit {
            retry_after,
            message: message.into(),
        }
    }

    /// Create a new unsupported network error
    pub fn unsupported_network<S: Into<String>>(network: S, feature: S) -> Self {
        EtherscanError::UnsupportedNetwork {
            network: network.into(),
            feature: feature.into(),
        }
    }

    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            EtherscanError::Request(_) |
            EtherscanError::Http { status, .. } if *status >= 500 ||
            EtherscanError::RateLimit { .. } ||
            EtherscanError::Timeout(_)
        )
    }

    /// Get the error category for logging/metrics
    pub fn category(&self) -> &'static str {
        match self {
            EtherscanError::MissingApiKey => "configuration",
            EtherscanError::InvalidUrl(_) => "configuration",
            EtherscanError::HttpClient(_) => "configuration",
            EtherscanError::Request(_) => "network",
            EtherscanError::Http { .. } => "http",
            EtherscanError::Api { .. } => "api",
            EtherscanError::Response(_) => "parsing",
            EtherscanError::Parse(_) => "parsing",
            EtherscanError::InvalidAddress(_) => "validation",
            EtherscanError::InvalidTxHash(_) => "validation",
            EtherscanError::InvalidBlock(_) => "validation",
            EtherscanError::RateLimit { .. } => "rate_limit",
            EtherscanError::Timeout(_) => "timeout",
            EtherscanError::InvalidParams(_) => "validation",
            EtherscanError::UnsupportedNetwork { .. } => "configuration",
            EtherscanError::Internal(_) => "internal",
        }
    }
}

impl fmt::Display for EtherscanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EtherscanError::MissingApiKey => {
                write!(f, "API key is required but not provided")
            }
            EtherscanError::InvalidUrl(msg) => {
                write!(f, "Invalid URL: {}", msg)
            }
            EtherscanError::HttpClient(msg) => {
                write!(f, "HTTP client error: {}", msg)
            }
            EtherscanError::Request(msg) => {
                write!(f, "Request failed: {}", msg)
            }
            EtherscanError::Http { status, message } => {
                write!(f, "HTTP error {}: {}", status, message)
            }
            EtherscanError::Api { message, result } => {
                match result {
                    Some(result) => write!(f, "API error: {} (result: {})", message, result),
                    None => write!(f, "API error: {}", message),
                }
            }
            EtherscanError::Response(msg) => {
                write!(f, "Response error: {}", msg)
            }
            EtherscanError::Parse(msg) => {
                write!(f, "Parse error: {}", msg)
            }
            EtherscanError::InvalidAddress(addr) => {
                write!(f, "Invalid Ethereum address: {}", addr)
            }
            EtherscanError::InvalidTxHash(hash) => {
                write!(f, "Invalid transaction hash: {}", hash)
            }
            EtherscanError::InvalidBlock(block) => {
                write!(f, "Invalid block identifier: {}", block)
            }
            EtherscanError::RateLimit { retry_after, message } => {
                match retry_after {
                    Some(seconds) => write!(f, "Rate limit exceeded: {} (retry after {} seconds)", message, seconds),
                    None => write!(f, "Rate limit exceeded: {}", message),
                }
            }
            EtherscanError::Timeout(msg) => {
                write!(f, "Request timeout: {}", msg)
            }
            EtherscanError::InvalidParams(msg) => {
                write!(f, "Invalid parameters: {}", msg)
            }
            EtherscanError::UnsupportedNetwork { network, feature } => {
                write!(f, "Feature '{}' is not supported on network '{}'", feature, network)
            }
            EtherscanError::Internal(msg) => {
                write!(f, "Internal error: {}", msg)
            }
        }
    }
}

impl std::error::Error for EtherscanError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

// Convenient conversion from common error types
impl From<reqwest::Error> for EtherscanError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            EtherscanError::Timeout(err.to_string())
        } else if err.is_request() {
            EtherscanError::Request(err.to_string())
        } else {
            EtherscanError::Request(err.to_string())
        }
    }
}

impl From<serde_json::Error> for EtherscanError {
    fn from(err: serde_json::Error) -> Self {
        EtherscanError::Parse(err.to_string())
    }
}

impl From<url::ParseError> for EtherscanError {
    fn from(err: url::ParseError) -> Self {
        EtherscanError::InvalidUrl(err.to_string())
    }
}

/// Helper functions for validation
pub mod validation {
    use super::EtherscanError;

    /// Validate Ethereum address format (basic check)
    pub fn validate_address(address: &str) -> Result<(), EtherscanError> {
        if !address.starts_with("0x") || address.len() != 42 {
            return Err(EtherscanError::InvalidAddress(address.to_string()));
        }

        // Check if all characters after 0x are hex
        if !address[2..].chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(EtherscanError::InvalidAddress(address.to_string()));
        }

        Ok(())
    }

    /// Validate transaction hash format
    pub fn validate_tx_hash(hash: &str) -> Result<(), EtherscanError> {
        if !hash.starts_with("0x") || hash.len() != 66 {
            return Err(EtherscanError::InvalidTxHash(hash.to_string()));
        }

        if !hash[2..].chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(EtherscanError::InvalidTxHash(hash.to_string()));
        }

        Ok(())
    }

    /// Validate block hash format
    pub fn validate_block_hash(hash: &str) -> Result<(), EtherscanError> {
        validate_tx_hash(hash).map_err(|_| EtherscanError::InvalidBlock(hash.to_string()))
    }

    /// Normalize address to lowercase
    pub fn normalize_address(address: &str) -> Result<String, EtherscanError> {
        validate_address(address)?;
        Ok(address.to_lowercase())
    }
}