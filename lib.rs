//! # keion-etherscan
//!
//! A Rust wrapper for the Etherscan API providing type-safe access to Ethereum blockchain data.
//!
//! ## Quick Start
//!
//! ```rust
//! use keion_etherscan::{EtherscanClient, Network};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = EtherscanClient::builder()
//!         .api_key("your-api-key")
//!         .network(Network::Mainnet)
//!         .build()?;
//!
//!     let balance = client.accounts().balance("0x...").await?;
//!     println!("Balance: {} ETH", balance);
//!     Ok(())
//! }
//! ```

// Re-exports for public API
pub use client::{EtherscanClient, EtherscanClientBuilder};
pub use error::{EtherscanError, Result};
pub use types::{Network, Sort, Tag, BlockType};

// Re-export key models that users will work with
pub use models::{
    Account, Transaction, Block, Contract, Token,
    Balance, TransactionReceipt, ContractSource,
};

// Module declarations
mod client;
mod error;
mod types;
mod utils;

pub mod endpoints;
pub mod models;

// Feature-gated exports
#[cfg(feature = "rate-limiting")]
pub use client::RateLimiter;

#[cfg(feature = "tracing")]
pub use tracing;

// Version and metadata
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const USER_AGENT: &str = concat!("keno-etherscan/", env!("CARGO_PKG_VERSION"));

// Convenience type aliases
pub type EtherscanResult<T> = Result<T, EtherscanError>;

// Optional: Prelude module for glob imports
pub mod prelude {
    pub use crate::{
        EtherscanClient, EtherscanError, Result,
        Network, Sort, Tag, BlockType,
    };
}