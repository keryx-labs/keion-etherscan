use std::time::Duration;
use keion_etherscan::{EtherscanClient, EtherscanError, Network};

#[test]
fn test_builder_pattern() {
    let builder = EtherscanClient::builder()
        .api_key("test-key")
        .network(Network::Goerli)
        .timeout(Duration::from_secs(60));

    // We can't directly access the builder fields due to privacy,
    // but we can test that the build succeeds
    let client = builder.build().unwrap();
    assert_eq!(client.network(), Network::Goerli);
}

#[test]
fn test_missing_api_key() {
    let result = EtherscanClient::builder().build();
    assert!(matches!(result, Err(EtherscanError::MissingApiKey)));
}

#[test]
fn test_api_key_preview() {
    let client = EtherscanClient::builder()
        .api_key("1234567890abcdef")
        .build()
        .unwrap();

    assert_eq!(client.api_key_preview(), "1234...cdef");
}

#[test]
fn test_short_api_key_preview() {
    let client = EtherscanClient::builder()
        .api_key("short")
        .build()
        .unwrap();

    assert_eq!(client.api_key_preview(), "****");
}

#[test]
fn test_simple_constructor() {
    let client = EtherscanClient::new("test-api-key").unwrap();
    assert_eq!(client.network(), Network::Mainnet);
}

#[test]
fn test_network_setting() {
    let client = EtherscanClient::builder()
        .api_key("test-key")
        .network(Network::Polygon)
        .build()
        .unwrap();

    assert_eq!(client.network(), Network::Polygon);
}

#[test]
fn test_default_builder() {
    let builder = EtherscanClient::builder();
    let client = builder.api_key("test-key").build().unwrap();
    assert_eq!(client.network(), Network::Mainnet); // Default network
}