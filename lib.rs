//! # keion-etherscan
//!
//! A comprehensive Rust wrapper for the Etherscan API family, providing type-safe
//! access to Ethereum blockchain data across multiple networks.
//!
//! ## Features
//!
//! - **Multi-network support**: Ethereum mainnet, testnets, and popular L2s
//! - **Type-safe**: Strongly typed responses using Rust's type system
//! - **Async/await**: Built on `reqwest` and `tokio` for async operations
//! - **Builder patterns**: Ergonomic API for constructing queries
//! - **Error handling**: Comprehensive error types with context
//! - **Rate limiting**: Built-in respect for API rate limits
//! - **Pagination**: Easy handling of paginated responses
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use keion_etherscan::{EtherscanClient, Network};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client
//!     let client = EtherscanClient::builder()
//!         .api_key("your-api-key-here")
//!         .network(Network::Mainnet)
//!         .build()?;
//!
//!     // Get account balance
//!     let balance = client
//!         .accounts()
//!         .balance("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6")
//!         .await?;
//!
//!     println!("Balance: {} ETH", balance.eth().unwrap_or(0.0));
//!
//!     // Get recent transactions
//!     let transactions = client
//!         .accounts()
//!         .transactions("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6")
//!         .page(1)
//!         .offset(10)
//!         .execute()
//!         .await?;
//!
//!     println!("Found {} transactions", transactions.len());
//!
//!     // Get token transfers
//!     let token_transfers = client
//!         .accounts()
//!         .token_transfers("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6")
//!         .contract_address("0xa0b86a33e6411b7a0a6acc95b0e8fd65b7b1b6c8")
//!         .page(1)
//!         .offset(100)
//!         .execute()
//!         .await?;
//!
//!     println!("Found {} token transfers", token_transfers.len());
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Supported Networks
//!
//! - Ethereum Mainnet
//! - Goerli Testnet
//! - Sepolia Testnet
//! - Binance Smart Chain
//! - Polygon
//! - Fantom
//! - Arbitrum
//! - Optimism
//!
//! ## API Coverage
//!
//! ### Accounts
//! - ETH balances (single and multi-address)
//! - Transaction history (normal, internal, token transfers)
//! - Token balances and transfers (ERC-20, ERC-721, ERC-1155)
//!
//! ### Transactions
//! - Transaction details and receipts
//! - Transaction status and confirmations
//!
//! ### Contracts
//! - Contract source code
//! - Contract ABI
//! - Contract verification status
//!
//! ### Blocks
//! - Block information
//! - Block rewards
//!
//! ### Tokens
//! - Token information
//! - Token supply
//!
//! ### Statistics
//! - Network statistics
//! - Gas prices

#![warn(missing_docs)]
#![warn(clippy::all)]

// Re-exports for public API
pub use client::{EtherscanClient, EtherscanClientBuilder};
pub use error::{EtherscanError, Result};
pub use types::{Network, Sort, Tag, BlockType, TransactionType, Pagination};

// Re-export key models that users will work with
pub use models::{
    // Account models
    Balance, TokenBalance, AccountInfo, MultiBalance, ValidatedBlock, BeaconWithdrawal,
    // Transaction models
    Transaction, InternalTransaction, TokenTransfer, TransactionReceipt, TransactionLog,
    // Common model types
    Address, TxHash, BigNumber, StringNumber, HexNumber,
};

// Module declarations
mod client;
pub mod error;
mod types;

pub mod endpoints;
pub mod models;

// Feature-gated exports
// #[cfg(feature = "rate-limiting")]
// pub use client::RateLimiter;

// Version and metadata
/// Current version of the keion-etherscan crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// User agent string used for HTTP requests
pub const USER_AGENT: &str = concat!("keion-etherscan/", env!("CARGO_PKG_VERSION"));

/// Convenience type alias for results
pub type EtherscanResult<T> = Result<T>;

/// Prelude module for convenient imports
pub mod prelude {
    //! Convenient re-exports for common usage
    pub use crate::{
        EtherscanClient, EtherscanError, Result,
        Network, Sort, Tag, BlockType,
        Balance, Transaction, TokenTransfer,
        Address, TxHash,
    };
}