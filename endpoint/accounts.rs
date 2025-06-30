use crate::{
    EtherscanClient, Result, EtherscanError,
    types::{Sort, Tag, Pagination, TransactionType},
    models::{Balance, Transaction, TokenBalance, TokenTransfer},
    error::validation::{validate_address, normalize_address},
    endpoints::EndpointGroup,
};

/// Account-related API endpoints
#[derive(Debug)]
pub struct Accounts<'a> {
    client: &'a EtherscanClient,
}

impl<'a> EndpointGroup for Accounts<'a> {
    fn new(client: &'a EtherscanClient) -> Self {
        Self { client }
    }

    fn client(&self) -> &EtherscanClient {
        self.client
    }
}

impl<'a> Accounts<'a> {
    /// Get ETH balance for a single address
    ///
    /// # Arguments
    /// * `address` - Ethereum address to query
    /// * `tag` - Block tag (latest, earliest, pending, or specific block number)
    ///
    /// # Example
    /// ```rust
    /// let balance = client.accounts()
    ///     .balance("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6")
    ///     .await?;
    /// ```
    pub async fn balance<S: AsRef<str>>(&self, address: S) -> Result<Balance> {
        let address = normalize_address(address.as_ref())?;

        let params = [
            ("address", address.as_str()),
            ("tag", "latest"),
        ];

        self.client.get("account", "balance", &params).await
    }

    /// Get ETH balance for a single address at a specific block
    pub async fn balance_at_block<S: AsRef<str>>(&self, address: S, tag: Tag) -> Result<Balance> {
        let address = normalize_address(address.as_ref())?;
        let tag_str = tag.as_str();

        let params = [
            ("address", address.as_str()),
            ("tag", tag_str.as_str()),
        ];

        self.client.get("account", "balance", &params).await
    }

    /// Get ETH balance for multiple addresses (up to 20)
    ///
    /// # Arguments
    /// * `addresses` - List of Ethereum addresses (max 20)
    ///
    /// # Example
    /// ```rust
    /// let balances = client.accounts()
    ///     .balance_multi(&[
    ///         "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6",
    ///         "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a7"
    ///     ])
    ///     .await?;
    /// ```
    pub async fn balance_multi<S: AsRef<str>>(&self, addresses: &[S]) -> Result<Vec<Balance>> {
        if addresses.is_empty() {
            return Err(EtherscanError::InvalidParams("At least one address required".to_string()));
        }

        if addresses.len() > 20 {
            return Err(EtherscanError::InvalidParams("Maximum 20 addresses allowed".to_string()));
        }

        // Validate and normalize all addresses
        let normalized: Result<Vec<String>> = addresses
            .iter()
            .map(|addr| normalize_address(addr.as_ref()))
            .collect();
        let normalized = normalized?;

        let address_list = normalized.join(",");
        let params = [
            ("address", address_list.as_str()),
            ("tag", "latest"),
        ];

        self.client.get("account", "balancemulti", &params).await
    }

    /// Get normal transactions for an address
    ///
    /// # Arguments
    /// * `address` - Ethereum address to query
    ///
    /// Returns a `TransactionQueryBuilder` for further configuration
    pub fn transactions<S: AsRef<str>>(&self, address: S) -> TransactionQueryBuilder<'a> {
        TransactionQueryBuilder::new(self.client, address.as_ref(), TransactionType::Normal)
    }

    /// Get internal transactions for an address
    pub fn internal_transactions<S: AsRef<str>>(&self, address: S) -> TransactionQueryBuilder<'a> {
        TransactionQueryBuilder::new(self.client, address.as_ref(), TransactionType::Internal)
    }

    /// Get ERC-20 token transfers for an address
    pub fn token_transfers<S: AsRef<str>>(&self, address: S) -> TokenTransferQueryBuilder<'a> {
        TokenTransferQueryBuilder::new(self.client, address.as_ref(), TransactionType::Token)
    }

    /// Get ERC-721 NFT transfers for an address
    pub fn nft_transfers<S: AsRef<str>>(&self, address: S) -> TokenTransferQueryBuilder<'a> {
        TokenTransferQueryBuilder::new(self.client, address.as_ref(), TransactionType::TokenNft)
    }

    /// Get ERC-1155 token transfers for an address
    pub fn erc1155_transfers<S: AsRef<str>>(&self, address: S) -> TokenTransferQueryBuilder<'a> {
        TokenTransferQueryBuilder::new(self.client, address.as_ref(), TransactionType::Token1155)
    }

    /// Get list of ERC-20 tokens owned by an address
    pub async fn token_balances<S: AsRef<str>>(&self, address: S) -> Result<Vec<TokenBalance>> {
        let address = normalize_address(address.as_ref())?;

        let params = [
            ("address", address.as_str()),
            ("tag", "latest"),
        ];

        self.client.get("account", "tokenlist", &params).await
    }
}

/// Builder for transaction queries with pagination and filtering
#[derive(Debug)]
pub struct TransactionQueryBuilder<'a> {
    client: &'a EtherscanClient,
    address: String,
    tx_type: TransactionType,
    pagination: Pagination,
}

impl<'a> TransactionQueryBuilder<'a> {
    fn new(client: &'a EtherscanClient, address: &str, tx_type: TransactionType) -> Self {
        Self {
            client,
            address: address.to_string(),
            tx_type,
            pagination: Pagination::new(),
        }
    }

    /// Set the page number (starting from 1)
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

    /// Set the sort order
    pub fn sort(mut self, sort: Sort) -> Self {
        self.pagination = self.pagination.sort(sort);
        self
    }

    /// Set a block range
    pub fn block_range(mut self, start: u64, end: u64) -> Self {
        self.pagination = self.pagination.block_range(start, end);
        self
    }

    /// Execute the query
    pub async fn execute(self) -> Result<Vec<Transaction>> {
        let address = normalize_address(&self.address)?;

        let mut params = vec![("address", address)];

        // Add pagination parameters
        let pagination_params = self.pagination.to_params();
        for (key, value) in pagination_params {
            params.push((key, value));
        }

        // Convert to &str tuples for the API call
        let params_ref: Vec<(&str, &str)> = params
            .iter()
            .map(|(k, v)| (*k, v.as_str()))
            .collect();

        self.client.get("account", self.tx_type.as_str(), &params_ref).await
    }
}

/// Builder for token transfer queries
#[derive(Debug)]
pub struct TokenTransferQueryBuilder<'a> {
    client: &'a EtherscanClient,
    address: String,
    tx_type: TransactionType,
    contract_address: Option<String>,
    pagination: Pagination,
}

impl<'a> TokenTransferQueryBuilder<'a> {
    fn new(client: &'a EtherscanClient, address: &str, tx_type: TransactionType) -> Self {
        Self {
            client,
            address: address.to_string(),
            tx_type,
            contract_address: None,
            pagination: Pagination::new(),
        }
    }

    /// Filter by specific token contract address
    pub fn contract_address<S: AsRef<str>>(mut self, contract_address: S) -> Self {
        self.contract_address = Some(contract_address.as_ref().to_string());
        self
    }

    /// Set pagination options
    pub fn page(mut self, page: u32) -> Self {
        self.pagination = self.pagination.page(page);
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.pagination = self.pagination.offset(offset);
        self
    }

    pub fn start_block(mut self, block: u64) -> Self {
        self.pagination = self.pagination.start_block(block);
        self
    }

    pub fn end_block(mut self, block: u64) -> Self {
        self.pagination = self.pagination.end_block(block);
        self
    }

    pub fn sort(mut self, sort: Sort) -> Self {
        self.pagination = self.pagination.sort(sort);
        self
    }

    pub fn block_range(mut self, start: u64, end: u64) -> Self {
        self.pagination = self.pagination.block_range(start, end);
        self
    }

    /// Execute the query
    pub async fn execute(self) -> Result<Vec<TokenTransfer>> {
        let address = normalize_address(&self.address)?;

        let mut params = vec![("address", address)];

        // Add contract address if specified
        if let Some(contract_addr) = self.contract_address {
            let normalized_contract = normalize_address(&contract_addr)?;
            params.push(("contractaddress", normalized_contract));
        }

        // Add pagination parameters
        let pagination_params = self.pagination.to_params();
        for (key, value) in pagination_params {
            params.push((key, value));
        }

        // Convert to &str tuples for the API call
        let params_ref: Vec<(&str, &str)> = params
            .iter()
            .map(|(k, v)| (*k, v.as_str()))
            .collect();

        self.client.get("account", self.tx_type.as_str(), &params_ref).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EtherscanClient, Network};

    fn create_test_client() -> EtherscanClient {
        EtherscanClient::builder()
            .api_key("test-key")
            .network(Network::Mainnet)
            .build()
            .unwrap()
    }

    #[test]
    fn test_transaction_query_builder() {
        let client = create_test_client();
        let accounts = client.accounts();

        let query = accounts
            .transactions("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6")
            .page(1)
            .offset(100)
            .start_block(1000)
            .end_block(2000)
            .sort(Sort::Ascending);

        assert_eq!(query.pagination.page, Some(1));
        assert_eq!(query.pagination.offset, Some(100));
        assert_eq!(query.pagination.start_block, Some(1000));
        assert_eq!(query.pagination.end_block, Some(2000));
    }

    #[test]
    fn test_token_transfer_query_builder() {
        let client = create_test_client();
        let accounts = client.accounts();

        let query = accounts
            .token_transfers("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6")
            .contract_address("0xa0b86a33e6411b7a0a6acc95b0e8fd65b7b1b6c8")
            .page(2)
            .offset(50);

        assert!(query.contract_address.is_some());
        assert_eq!(query.pagination.page, Some(2));
        assert_eq!(query.pagination.offset, Some(50));
    }
}