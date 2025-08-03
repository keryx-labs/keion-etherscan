//! API endpoint implementations for the Etherscan client

use crate::EtherscanClient;

pub mod accounts;
pub mod blocks;
pub mod contracts;
pub mod stats;
pub mod tokens;
pub mod transactions;

pub use accounts::Accounts;
pub use blocks::Blocks;
pub use contracts::Contracts;
pub use stats::Stats;
pub use tokens::Tokens;
pub use transactions::Transactions;

/// Base trait for all endpoint groups
pub trait EndpointGroup {
    fn new(client: &EtherscanClient) -> Self
    where
        Self: Sized;
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
