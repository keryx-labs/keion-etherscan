//! API endpoint implementations for the Etherscan client

use crate::EtherscanClient;

pub mod accounts;
pub mod transactions;
pub mod contracts;
pub mod blocks;
pub mod tokens;
pub mod stats;

pub use accounts::Accounts;
pub use transactions::Transactions;
pub use contracts::Contracts;
pub use blocks::Blocks;
pub use tokens::Tokens;
pub use stats::Stats;

/// Base trait for all endpoint groups
pub trait EndpointGroup {
    fn new(client: &EtherscanClient) -> Self;
}

/// Macro to implement common endpoint functionality
macro_rules! impl_endpoint {
    ($name:ident) => {
        impl<'a> $name<'a> {
            pub fn new(client: &'a EtherscanClient) -> Self {
                Self { client }
            }
        }

        impl<'a> EndpointGroup for $name<'a> {
            fn new(client: &EtherscanClient) -> Self {
                Self::new(client)
            }
        }
    };
}

pub(crate) use impl_endpoint;