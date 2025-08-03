use crate::EtherscanClient;

/// Transaction-related API endpoints
#[derive(Debug)]
pub struct Transactions<'a> {
    client: &'a EtherscanClient,
}

impl<'a> Transactions<'a> {
    pub fn new(client: &'a EtherscanClient) -> Self {
        Self { client }
    }

    // TODO: Implement transaction endpoints
}
