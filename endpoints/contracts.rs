use crate::EtherscanClient;

/// Contract-related API endpoints
#[derive(Debug)]
pub struct Contracts<'a> {
    client: &'a EtherscanClient,
}

impl<'a> Contracts<'a> {
    pub fn new(client: &'a EtherscanClient) -> Self {
        Self { client }
    }
    
    // TODO: Implement contract endpoints
}