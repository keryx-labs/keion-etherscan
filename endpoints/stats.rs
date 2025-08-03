use crate::EtherscanClient;

/// Stats-related API endpoints
#[derive(Debug)]
pub struct Stats<'a> {
    client: &'a EtherscanClient,
}

impl<'a> Stats<'a> {
    pub fn new(client: &'a EtherscanClient) -> Self {
        Self { client }
    }

    // TODO: Implement stats endpoints
}
