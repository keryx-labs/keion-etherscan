use keion_etherscan::{EtherscanClient, Network};

#[test]
fn test_basic_client_creation() {
    let client = EtherscanClient::builder()
        .api_key("test-key")
        .network(Network::Mainnet)
        .build();

    assert!(client.is_ok());
}

#[test]
fn test_accounts_endpoint_access() {
    let client = EtherscanClient::builder()
        .api_key("test-key")
        .network(Network::Mainnet)
        .build()
        .unwrap();

    let accounts = client.accounts();
    let _query = accounts.transactions("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6");

    // Test passes if it compiles
}
