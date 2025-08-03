use crate::EtherscanClient;

/// Token-related API endpoints
#[derive(Debug)]
pub struct Tokens<'a> {
    client: &'a EtherscanClient,
}

impl<'a> Tokens<'a> {
    pub fn new(client: &'a EtherscanClient) -> Self {
        Self { client }
    }
    
    // TODO: Implement token endpoints
}