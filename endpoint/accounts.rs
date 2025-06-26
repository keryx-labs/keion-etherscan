use crate::{
    error::{validation, Result},
    types::{Pagination, Sort, Tag, TransactionType},
    models::{Balance, Transaction, TokenTransfer},
    EtherscanClient,
};

use super::impl_endpoint;

/// Account-related API endpoints
#[derive(Debug)]
pub struct Accounts<'a> {
    client: &'a EtherscanClient,
}

impl_endpoint!(Accounts);

impl<'a> Accounts<'a> {
    /// Get ETH balance for a single address
    ///
    /// # Example
    /// ```rust
    /// let balance = client.accounts()
    ///     .balance("0x742d35Cc6634C0532925a3b8D39341Ce2E4bbF3B")
    ///     .await?;
    /// ```
    pub async fn balance(&self, address: &str) -> Result<Balance> {
        let address = validation::normalize_address(address)?;

        let params = [
            ("address", address.as_str()),
            ("tag", "latest"),
        ];

        self.client.get("account", "balance", &params).await
    }

    /// Get ETH balance for multiple addresses (up to 20)
    ///
    /// # Example
    /// ```rust
    /// let addresses = vec![
    ///     "0x742d35Cc6634C0532925a3b8D39341Ce2E4bbF3B",
    ///     "0x9E5E5F5C5C5C5C5C5C5C5C5C5C5C5C5C5C5C5C5C",
    /// ];
    /// let balances = client.accounts().balances(&addresses).await?;
    /// ```
    pub async fn balances(&self, addresses: &[&str]) -> Result<Vec<Balance>> {
        if addresses.is_empty() {
            return Ok(Vec::new());
        }

        if addresses.len() > 20 {
            return Err(crate::error::EtherscanError::InvalidParams(
                "Maximum 20 addresses allowed".to_string()
            ));
        }

        // Validate and normalize all addresses
        let normalized_addresses: Result<Vec<String>> = addresses
            .iter()
            .map(|addr| validation::normalize_address(addr))
            .collect();

        let normalized_addresses = normalized_addresses?;
        let addresses_str = normalized_addresses.join(",");

        let params = [
            ("address", addresses_str.as_str()),
            ("tag", "latest"),
        ];

        self.client.get("account", "balancemulti", &params).await
    }

    /// Get list of normal transactions for an address
    ///
    /// # Example
    /// ```rust
    /// let txs = client.accounts()
    ///     .transactions("0x742d35Cc6634C0532925a3b8D39341Ce2E4bbF3B")
    ///     .page(1)
    ///     .offset(100)
    ///     .sort(Sort::Descending)
    ///     .await?;
    /// ```
    pub fn transactions(&self, address: &str) -> TransactionQuery<'a> {
        TransactionQuery::new(self.client, address, TransactionType::Normal)
    }

    /// Get list of internal transactions for an address
    pub fn internal_transactions(&self, address: &str) -> TransactionQuery<'a> {
        TransactionQuery::new(self.client, address, TransactionType::Internal)
    }

    /// Get list of ERC-20 token transfers for an address
    pub fn token_transfers(&self, address: &str) -> TokenTransferQuery<'a> {
        TokenTransferQuery::new(self.client, address, None)
    }

    /// Get list of ERC-721 NFT transfers for an address
    pub fn nft_transfers(&self, address: &str) -> TokenTransferQuery<'a> {
        TokenTransferQuery::new(self.client, address, Some("erc721"))
    }

    /// Get list of ERC-1155 token transfers for an address
    pub fn erc1155_transfers(&self, address: &str) -> TokenTransferQuery<'a> {
        TokenTransferQuery::new(self.client, address, Some("erc1155"))
    }

    /// Get historical ETH balance for an address at a specific block
    pub async fn balance_at_block(&self, address: &str, block: Tag) -> Result<Balance> {
        let address = validation::normalize_address(address)?;

        let params = [
            ("address", address.as_str()),
            ("tag", &block.as_str()),
        ];

        self.client.get("account", "balance", &params).await
    }
}

/// Query builder for transaction requests
#[derive(Debug)]
pub struct TransactionQuery<'a> {
    client: &'a EtherscanClient,
    address: String,
    tx_type: TransactionType,
    pagination: Pagination,
}

impl<'a> TransactionQuery<'a> {
    fn new(client: &'a EtherscanClient, address: &str, tx_type: TransactionType) -> Self {
        Self {
            client,
            address: address.to_string(), // Will be validated when executed
            tx_type,
            pagination: Pagination::new(),
        }
    }

    /// Set the page number (starts from 1)
    pub fn page(mut self, page: u32) -> Self {
        self.pagination = self.pagination.page(page);
        self
    }

    /// Set the number of transactions per page (max 10000)
    pub fn offset(mut self, offset: u32) -> Self {
        self.pagination = self.pagination.offset(offset);
        self
    }

    /// Set the starting block number
    pub fn start_block(mut self, block: u64) -> Self {
        self.pagination = self.pagination.start_block(block);
        self
    }

    /// Set the ending block number
    pub fn end_block(mut self, block: u64) -> Self {
        self.pagination = self.pagination.end_block(block);
        self
    }

    /// Set the block range
    pub fn block_range(mut self, start: u64, end: u64) -> Self {
        self.pagination = self.pagination.block_range(start, end);
        self
    }

    /// Set the sort order
    pub fn sort(mut self, sort: Sort) -> Self {
        self.pagination = self.pagination.sort(sort);
        self
    }

    /// Execute the query and return transactions
    pub async fn await(self) -> Result<Vec<Transaction>> {
        let address = validation::normalize_address(&self.address)?;

        let mut params = vec![("address", address)];

        // Add pagination parameters
        for (key, value) in self.pagination.to_params() {
            params.push((key, value));
        }

        // Convert to string references for the API call
        let string_params: Vec<(&str, &str)> = params
            .iter()
            .map(|(k, v)| (*k, v.as_str()))
            .collect();

        self.client.get("account", self.tx_type.as_str(), &string_params).await
    }
}

/// Query builder for token transfer requests
#[derive(Debug)]
pub struct TokenTransferQuery<'a> {
    client: &'a EtherscanClient,
    address: String,
    token_type: Option<&'static str>,
    contract_address: Option<String>,
    pagination: Pagination,
}

impl<'a> TokenTransferQuery<'a> {
    fn new(client: &'a EtherscanClient, address: &str, token_type: Option<&'static str>) -> Self {
        Self {
            client,
            address: address.to_string(),
            token_type,
            contract_address: None,
            pagination: Pagination::new(),
        }
    }

    /// Filter by specific token contract address
    pub fn contract_address(mut self, contract_address: &str) -> Self {
        self.contract_address = Some(contract_address.to_string());
        self
    }

    /// Set the page number
    pub fn page(mut self, page: u32) -> Self {
        self.pagination = self.pagination.page(page);
        self
    }

    /// Set the number of transfers per page
    pub fn offset(mut self, offset: u32) -> Self {
        self.pagination = self.pagination.offset(offset);
        self
    }

    /// Set the starting block number
    pub fn start_block(mut self, block: u64) -> Self {
        self.pagination = self.pagination.start_block(block);
        self
    }

    /// Set the ending block number
    pub fn end_block(mut self, block: u64) -> Self {
        self.pagination = self.pagination.end_block(block);
        self
    }

    /// Set the sort order
    pub fn sort(mut self, sort: Sort) -> Self {
        self.pagination = self.pagination.sort(sort);
        self
    }

    /// Execute the query and return token transfers
    pub async fn await(self) -> Result<Vec<TokenTransfer>> {
        let address = validation::normalize_address(&self.address)?;

        let mut params = vec![("address", address)];

        // Add contract address if specified
        if let Some(contract_addr) = &self.contract_address {
            let normalized_contract = validation::normalize_address(contract_addr)?;
            params.push(("contractaddress", normalized_contract));
        }

        // Add pagination parameters
        for (key, value) in self.pagination.to_params() {
            params.push((key, value));
        }

        // Determine the action based on token type
        let action = match self.token_type {
            Some("erc721") => "tokennfttx",
            Some("erc1155") => "token1155tx",
            _ => "tokentx", // Default to ERC-20
        };

        // Convert to string references for the API call
        let string_params: Vec<(&str, &str)> = params
            .iter()
            .map(|(k, v)| (*k, v.as_str()))
            .collect();

        self.client.get("account", action, &string_params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EtherscanClient, Network};

    #[tokio::test]
    async fn test_balance_query_builder() {
        let client = EtherscanClient::builder()
            .api_key("test")
            .network(Network::Mainnet)
            .build()
            .unwrap();

        let accounts = client.accounts();

        // Test that the query builder compiles and has the right methods
        let _query = accounts
            .transactions("0x742d35Cc6634C0532925a3b8D39341Ce2E4bbF3B")
            .page(1)
            .offset(100)
            .sort(Sort::Descending);

        // Note: We can't actually execute without a real API key
    }

    #[test]
    fn test_multiple_addresses_validation() {
        // Test that we properly validate the address limit
        let addresses: Vec<&str> = (0..25).map(|_| "0x742d35Cc6634C0532925a3b8D39341Ce2E4bbF3B").collect();

        // This should fail validation (too many addresses)
        // In a real test, we'd construct a client and call balances()
    }
}