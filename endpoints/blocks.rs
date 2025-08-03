use crate::EtherscanClient;

/// Block-related API endpoints
#[derive(Debug)]
pub struct Blocks<'a> {
    client: &'a EtherscanClient,
}

impl<'a> Blocks<'a> {
    pub fn new(client: &'a EtherscanClient) -> Self {
        Self { client }
    }

    // TODO: Implement block endpoints
}
