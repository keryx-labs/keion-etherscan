use std::time::Duration;
use reqwest::Client;
use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    error::{EtherscanError, Result},
    types::{Network, EtherscanResponse},
    endpoints::{Accounts, Transactions, Contracts, Blocks, Tokens, Stats},
};

/// Main client for interacting with the Etherscan API
#[derive(Debug, Clone)]
pub struct EtherscanClient {
    http_client: Client,
    api_key: String,
    base_url: Url,
    network: Network,
}

/// Builder for configuring the Etherscan client
#[derive(Debug)]
pub struct EtherscanClientBuilder {
    api_key: Option<String>,
    network: Network,
    timeout: Option<Duration>,
    user_agent: Option<String>,
    rate_limit: Option<u32>,
}

impl EtherscanClientBuilder {
    /// Create a new builder with default settings
    pub fn new() -> Self {
        Self {
            api_key: None,
            network: Network::Mainnet,
            timeout: Some(Duration::from_secs(30)),
            user_agent: Some(format!("keion-etherscan/{}", env!("CARGO_PKG_VERSION"))),
            rate_limit: Some(5), // 5 requests per second default
        }
    }

    /// Set the API key (required)
    pub fn api_key<S: Into<String>>(mut self, api_key: S) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Set the network (default: Mainnet)
    pub fn network(mut self, network: Network) -> Self {
        self.network = network;
        self
    }

    /// Set request timeout (default: 30 seconds)
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set custom user agent
    pub fn user_agent<S: Into<String>>(mut self, user_agent: S) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Set rate limit in requests per second (default: 5)
    pub fn rate_limit(mut self, requests_per_second: u32) -> Self {
        self.rate_limit = Some(requests_per_second);
        self
    }

    /// Build the client
    pub fn build(self) -> Result<EtherscanClient> {
        let api_key = self.api_key.ok_or(EtherscanError::MissingApiKey)?;

        let base_url = self.network.base_url()
            .parse()
            .map_err(|e| EtherscanError::InvalidUrl(format!("Invalid base URL: {}", e)))?;

        let mut client_builder = Client::builder();

        if let Some(timeout) = self.timeout {
            client_builder = client_builder.timeout(timeout);
        }

        if let Some(user_agent) = self.user_agent {
            client_builder = client_builder.user_agent(user_agent);
        }

        let http_client = client_builder
            .build()
            .map_err(|e| EtherscanError::HttpClient(e.to_string()))?;

        Ok(EtherscanClient {
            http_client,
            api_key,
            base_url,
            network: self.network,
        })
    }
}

impl Default for EtherscanClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl EtherscanClient {
    /// Create a new client builder
    pub fn builder() -> EtherscanClientBuilder {
        EtherscanClientBuilder::new()
    }

    /// Create a client with just an API key (uses mainnet and defaults)
    pub fn new<S: Into<String>>(api_key: S) -> Result<Self> {
        Self::builder().api_key(api_key).build()
    }

    /// Get the current network
    pub fn network(&self) -> Network {
        self.network
    }

    /// Get the API key (for debugging/logging)
    pub fn api_key_preview(&self) -> String {
        let key = &self.api_key;
        if key.len() > 8 {
            format!("{}...{}", &key[..4], &key[key.len()-4..])
        } else {
            "****".to_string()
        }
    }

    // API endpoint accessors
    /// Access account-related endpoints
    pub fn accounts(&self) -> Accounts {
        Accounts::new(self)
    }

    /// Access transaction-related endpoints
    pub fn transactions(&self) -> Transactions {
        Transactions::new(self)
    }

    /// Access contract-related endpoints
    pub fn contracts(&self) -> Contracts {
        Contracts::new(self)
    }

    /// Access block-related endpoints
    pub fn blocks(&self) -> Blocks {
        Blocks::new(self)
    }

    /// Access token-related endpoints
    pub fn tokens(&self) -> Tokens {
        Tokens::new(self)
    }

    /// Access stats-related endpoints
    pub fn stats(&self) -> Stats {
        Stats::new(self)
    }

    // Internal methods for making requests
    pub(crate) async fn get<T>(&self, module: &str, action: &str, params: &[(&str, &str)]) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self.make_request(module, action, params).await?;
        self.parse_response(response).await
    }

    pub(crate) async fn make_request(
        &self,
        module: &str,
        action: &str,
        params: &[(&str, &str)],
    ) -> Result<reqwest::Response> {
        let mut url = self.base_url.clone();

        // Add query parameters
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair("module", module);
            query_pairs.append_pair("action", action);
            query_pairs.append_pair("apikey", &self.api_key);

            for (key, value) in params {
                query_pairs.append_pair(key, value);
            }
        }

        let request = self.http_client.get(url);
        let response = request.send().await
            .map_err(|e| EtherscanError::Request(e.to_string()))?;

        if !response.status().is_success() {
            return Err(EtherscanError::Http {
                status: response.status().as_u16(),
                message: response.text().await.unwrap_or_else(|_| "Unknown error".to_string()),
            });
        }

        Ok(response)
    }

    async fn parse_response<T>(&self, response: reqwest::Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let text = response.text().await
            .map_err(|e| EtherscanError::Response(format!("Failed to read response: {}", e)))?;

        // First try to parse as EtherscanResponse wrapper
        match serde_json::from_str::<EtherscanResponse<T>>(&text) {
            Ok(wrapper) => {
                match wrapper.status.as_str() {
                    "1" => Ok(wrapper.result),
                    "0" => Err(EtherscanError::Api {
                        message: wrapper.message,
                        result: None,
                    }),
                    _ => Err(EtherscanError::Response(format!(
                        "Unknown status: {}", wrapper.status
                    ))),
                }
            }
            Err(_) => {
                // Fallback: try to parse directly as T
                serde_json::from_str(&text)
                    .map_err(|e| EtherscanError::Parse(format!("JSON parse error: {}", e)))
            }
        }
    }
}