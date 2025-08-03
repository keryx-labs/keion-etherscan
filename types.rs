use serde::{Deserialize, Serialize};

/// Supported Ethereum networks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Network {
    Mainnet,
    Goerli,
    Sepolia,
    #[serde(rename = "bsc")]
    BinanceSmartChain,
    #[serde(rename = "polygon")]
    Polygon,
    #[serde(rename = "fantom")]
    Fantom,
    #[serde(rename = "arbitrum")]
    Arbitrum,
    #[serde(rename = "optimism")]
    Optimism,
}

impl Network {
    /// Get the base URL for the network's Etherscan API
    pub fn base_url(&self) -> &'static str {
        match self {
            Network::Mainnet => "https://api.etherscan.io/api",
            Network::Goerli => "https://api-goerli.etherscan.io/api",
            Network::Sepolia => "https://api-sepolia.etherscan.io/api",
            Network::BinanceSmartChain => "https://api.bscscan.com/api",
            Network::Polygon => "https://api.polygonscan.com/api",
            Network::Fantom => "https://api.ftmscan.com/api",
            Network::Arbitrum => "https://api.arbiscan.io/api",
            Network::Optimism => "https://api-optimistic.etherscan.io/api",
        }
    }

    /// Get the human-readable name of the network
    pub fn name(&self) -> &'static str {
        match self {
            Network::Mainnet => "Ethereum Mainnet",
            Network::Goerli => "Goerli Testnet",
            Network::Sepolia => "Sepolia Testnet",
            Network::BinanceSmartChain => "Binance Smart Chain",
            Network::Polygon => "Polygon",
            Network::Fantom => "Fantom",
            Network::Arbitrum => "Arbitrum",
            Network::Optimism => "Optimism",
        }
    }

    /// Get the chain ID for the network
    pub fn chain_id(&self) -> u64 {
        match self {
            Network::Mainnet => 1,
            Network::Goerli => 5,
            Network::Sepolia => 11155111,
            Network::BinanceSmartChain => 56,
            Network::Polygon => 137,
            Network::Fantom => 250,
            Network::Arbitrum => 42161,
            Network::Optimism => 10,
        }
    }
}

impl Default for Network {
    fn default() -> Self {
        Network::Mainnet
    }
}

impl std::fmt::Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Sort order for API responses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Sort {
    #[serde(rename = "asc")]
    Ascending,
    #[serde(rename = "desc")]
    Descending,
}

impl Sort {
    pub fn as_str(&self) -> &'static str {
        match self {
            Sort::Ascending => "asc",
            Sort::Descending => "desc",
        }
    }
}

impl Default for Sort {
    fn default() -> Self {
        Sort::Descending
    }
}

/// Block tag for API requests
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tag {
    Latest,
    Earliest,
    Pending,
    Block(u64),
}

impl Tag {
    pub fn as_str(&self) -> String {
        match self {
            Tag::Latest => "latest".to_string(),
            Tag::Earliest => "earliest".to_string(),
            Tag::Pending => "pending".to_string(),
            Tag::Block(number) => number.to_string(),
        }
    }
}

impl Default for Tag {
    fn default() -> Self {
        Tag::Latest
    }
}

impl From<u64> for Tag {
    fn from(block_number: u64) -> Self {
        Tag::Block(block_number)
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Block type for certain API calls
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BlockType {
    Blocks,
    Uncles,
}

impl BlockType {
    pub fn as_str(&self) -> &'static str {
        match self {
            BlockType::Blocks => "blocks",
            BlockType::Uncles => "uncles",
        }
    }
}

/// Transaction type filter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Normal,
    Internal,
    #[serde(rename = "tokentx")]
    Token,
    #[serde(rename = "tokennfttx")]
    TokenNft,
    #[serde(rename = "token1155tx")]
    Token1155,
}

impl TransactionType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionType::Normal => "txlist",
            TransactionType::Internal => "txlistinternal",
            TransactionType::Token => "tokentx",
            TransactionType::TokenNft => "tokennfttx",
            TransactionType::Token1155 => "token1155tx",
        }
    }
}

/// Standard response wrapper from Etherscan API
#[derive(Debug, Clone, Deserialize)]
pub struct EtherscanResponse<T> {
    pub status: String,
    pub message: String,
    pub result: T,
}

/// Pagination parameters for API requests
#[derive(Debug, Clone, Default)]
pub struct Pagination {
    pub page: Option<u32>,
    pub offset: Option<u32>,
    pub start_block: Option<u64>,
    pub end_block: Option<u64>,
    pub sort: Option<Sort>,
}

impl Pagination {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn start_block(mut self, block: u64) -> Self {
        self.start_block = Some(block);
        self
    }

    pub fn end_block(mut self, block: u64) -> Self {
        self.end_block = Some(block);
        self
    }

    pub fn sort(mut self, sort: Sort) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn block_range(mut self, start: u64, end: u64) -> Self {
        self.start_block = Some(start);
        self.end_block = Some(end);
        self
    }

    /// Convert to query parameters
    pub fn to_params(&self) -> Vec<(&'static str, String)> {
        let mut params = Vec::new();

        if let Some(page) = self.page {
            params.push(("page", page.to_string()));
        }
        if let Some(offset) = self.offset {
            params.push(("offset", offset.to_string()));
        }
        if let Some(start_block) = self.start_block {
            params.push(("startblock", start_block.to_string()));
        }
        if let Some(end_block) = self.end_block {
            params.push(("endblock", end_block.to_string()));
        }
        if let Some(sort) = self.sort {
            params.push(("sort", sort.as_str().to_string()));
        }

        params
    }
}

/// Gas price levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GasPrice {
    Safe,
    Standard,
    Fast,
}

impl GasPrice {
    pub fn as_str(&self) -> &'static str {
        match self {
            GasPrice::Safe => "safegasprice",
            GasPrice::Standard => "standardgasprice",
            GasPrice::Fast => "fastgasprice",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_urls() {
        assert_eq!(Network::Mainnet.base_url(), "https://api.etherscan.io/api");
        assert_eq!(
            Network::Goerli.base_url(),
            "https://api-goerli.etherscan.io/api"
        );
        assert_eq!(
            Network::BinanceSmartChain.base_url(),
            "https://api.bscscan.com/api"
        );
    }

    #[test]
    fn test_network_chain_ids() {
        assert_eq!(Network::Mainnet.chain_id(), 1);
        assert_eq!(Network::Goerli.chain_id(), 5);
        assert_eq!(Network::Polygon.chain_id(), 137);
    }

    #[test]
    fn test_tag_conversion() {
        assert_eq!(Tag::Latest.as_str(), "latest");
        assert_eq!(Tag::Block(12345).as_str(), "12345");
        assert_eq!(Tag::from(98765).as_str(), "98765");
    }

    #[test]
    fn test_pagination_params() {
        let pagination = Pagination::new()
            .page(1)
            .offset(100)
            .start_block(1000)
            .end_block(2000)
            .sort(Sort::Ascending);

        let params = pagination.to_params();
        assert_eq!(params.len(), 5);
        assert!(params.contains(&("page", "1".to_string())));
        assert!(params.contains(&("offset", "100".to_string())));
        assert!(params.contains(&("sort", "asc".to_string())));
    }
}
