use crate::{
    EtherscanClient, Result, EtherscanError,
    types::{Sort, Tag, Pagination, TransactionType},
    models::{Balance, Transaction, TokenBalance, TokenTransfer, InternalTransaction, ValidatedBlock, BeaconWithdrawal},
    error::validation::normalize_address,
};

/// Account-related API endpoints
#[derive(Debug)]
pub struct Accounts<'a> {
    client: &'a EtherscanClient,
}


impl<'a> Accounts<'a> {
    pub fn new(client: &'a EtherscanClient) -> Self {
        Self { client }
    }
    /// Get ETH balance for a single address
    ///
    /// # Arguments
    /// * `address` - Ethereum address to query
    /// * `tag` - Block tag (latest, earliest, pending, or specific block number)
    ///
    /// # Example
    /// ```rust,no_run
    /// use keion_etherscan::EtherscanClient;
    /// 
    /// #[tokio::main]
    /// async fn main() -> keion_etherscan::Result<()> {
    ///     let client = EtherscanClient::new("YOUR_API_KEY")?;
    ///     let balance = client.accounts()
    ///         .balance("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6")
    ///         .await?;
    ///     Ok(())
    /// }
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
    /// ```rust,no_run
    /// use keion_etherscan::EtherscanClient;
    /// 
    /// #[tokio::main]
    /// async fn main() -> keion_etherscan::Result<()> {
    ///     let client = EtherscanClient::new("YOUR_API_KEY")?;
    ///     let balances = client.accounts()
    ///         .balance_multi(&[
    ///             "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6",
    ///             "0x742d35cc6634c0532925a3b8d19389c4d5e1e4a7"
    ///         ])
    ///         .await?;
    ///     Ok(())
    /// }
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

    /// Get internal transactions - returns a builder for various query types
    pub fn internal_transactions(&self) -> InternalTransactionQueryBuilder<'a> {
        InternalTransactionQueryBuilder::new(self.client)
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

    /// Get list of blocks validated by address (for validators)
    pub fn blocks_validated<S: AsRef<str>>(&self, address: S) -> ValidatedBlocksQueryBuilder<'a> {
        ValidatedBlocksQueryBuilder::new(self.client, address.as_ref())
    }

    /// Get beacon chain withdrawals for an address
    pub fn beacon_withdrawals<S: AsRef<str>>(&self, address: S) -> BeaconWithdrawalsQueryBuilder<'a> {
        BeaconWithdrawalsQueryBuilder::new(self.client, address.as_ref())
    }

    /// Get historical balance for a single address at a specific block
    pub fn historical_balance<S: AsRef<str>>(&self, address: S) -> HistoricalBalanceQueryBuilder<'a> {
        HistoricalBalanceQueryBuilder::new(self.client, address.as_ref())
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

    // Getter methods for testing
    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn get_tx_type(&self) -> TransactionType {
        self.tx_type
    }

    pub fn get_pagination(&self) -> &Pagination {
        &self.pagination
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

    // Getter methods for testing

    pub fn get_address(&self) -> &str {
        &self.address
    }


    pub fn get_tx_type(&self) -> TransactionType {
        self.tx_type
    }


    pub fn get_contract_address(&self) -> &Option<String> {
        &self.contract_address
    }


    pub fn get_pagination(&self) -> &Pagination {
        &self.pagination
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

/// Builder for internal transaction queries with multiple query types
#[derive(Debug)]
pub struct InternalTransactionQueryBuilder<'a> {
    client: &'a EtherscanClient,
}

impl<'a> InternalTransactionQueryBuilder<'a> {
    fn new(client: &'a EtherscanClient) -> Self {
        Self { client }
    }

    /// Query internal transactions by address
    pub fn by_address<S: AsRef<str>>(self, address: S) -> InternalTxByAddressBuilder<'a> {
        InternalTxByAddressBuilder::new(self.client, address.as_ref())
    }

    /// Query internal transactions by transaction hash
    pub fn by_hash<S: AsRef<str>>(self, tx_hash: S) -> InternalTxByHashBuilder<'a> {
        InternalTxByHashBuilder::new(self.client, tx_hash.as_ref())
    }

    /// Query internal transactions by block range
    pub fn by_block_range(self, start_block: u64, end_block: u64) -> InternalTxByBlockRangeBuilder<'a> {
        InternalTxByBlockRangeBuilder::new(self.client, start_block, end_block)
    }
}

/// Builder for internal transactions by address
#[derive(Debug)]
pub struct InternalTxByAddressBuilder<'a> {
    client: &'a EtherscanClient,
    address: String,
    pagination: Pagination,
}

impl<'a> InternalTxByAddressBuilder<'a> {
    fn new(client: &'a EtherscanClient, address: &str) -> Self {
        Self {
            client,
            address: address.to_string(),
            pagination: Pagination::new(),
        }
    }

    /// Set pagination and filtering options
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

    // Getter methods for testing

    pub fn get_address(&self) -> &str {
        &self.address
    }


    pub fn get_pagination(&self) -> &Pagination {
        &self.pagination
    }

    /// Execute the query
    pub async fn execute(self) -> Result<Vec<InternalTransaction>> {
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

        self.client.get("account", "txlistinternal", &params_ref).await
    }
}

/// Builder for internal transactions by transaction hash
#[derive(Debug)]
pub struct InternalTxByHashBuilder<'a> {
    client: &'a EtherscanClient,
    tx_hash: String,
}

impl<'a> InternalTxByHashBuilder<'a> {
    fn new(client: &'a EtherscanClient, tx_hash: &str) -> Self {
        Self {
            client,
            tx_hash: tx_hash.to_string(),
        }
    }

    // Getter methods for testing

    pub fn get_tx_hash(&self) -> &str {
        &self.tx_hash
    }

    /// Execute the query
    pub async fn execute(self) -> Result<Vec<InternalTransaction>> {
        let params = [("txhash", self.tx_hash.as_str())];

        self.client.get("account", "txlistinternal", &params).await
    }
}

/// Builder for internal transactions by block range
#[derive(Debug)]
pub struct InternalTxByBlockRangeBuilder<'a> {
    client: &'a EtherscanClient,
    start_block: u64,
    end_block: u64,
    pagination: Pagination,
}

impl<'a> InternalTxByBlockRangeBuilder<'a> {
    fn new(client: &'a EtherscanClient, start_block: u64, end_block: u64) -> Self {
        Self {
            client,
            start_block,
            end_block,
            pagination: Pagination::new().sort(Sort::Ascending),
        }
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

    pub fn sort(mut self, sort: Sort) -> Self {
        self.pagination = self.pagination.sort(sort);
        self
    }

    // Getter methods for testing

    pub fn get_start_block(&self) -> u64 {
        self.start_block
    }


    pub fn get_end_block(&self) -> u64 {
        self.end_block
    }


    pub fn get_pagination(&self) -> &Pagination {
        &self.pagination
    }

    /// Execute the query
    pub async fn execute(self) -> Result<Vec<InternalTransaction>> {
        let mut params = vec![
            ("startblock", self.start_block.to_string()),
            ("endblock", self.end_block.to_string()),
        ];

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

        self.client.get("account", "txlistinternal", &params_ref).await
    }
}

/// Builder for validated blocks query
#[derive(Debug)]
pub struct ValidatedBlocksQueryBuilder<'a> {
    client: &'a EtherscanClient,
    address: String,
    pagination: Pagination,
}

impl<'a> ValidatedBlocksQueryBuilder<'a> {
    fn new(client: &'a EtherscanClient, address: &str) -> Self {
        Self {
            client,
            address: address.to_string(),
            pagination: Pagination::new(),
        }
    }

    /// Set the page number
    pub fn page(mut self, page: u32) -> Self {
        self.pagination = self.pagination.page(page);
        self
    }

    /// Set the number of results per page
    pub fn offset(mut self, offset: u32) -> Self {
        self.pagination = self.pagination.offset(offset);
        self
    }

    // Getter methods for testing

    pub fn get_address(&self) -> &str {
        &self.address
    }


    pub fn get_pagination(&self) -> &Pagination {
        &self.pagination
    }

    /// Execute the query
    pub async fn execute(self) -> Result<Vec<ValidatedBlock>> {
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

        self.client.get("account", "getminedblocks", &params_ref).await
    }
}

/// Builder for beacon chain withdrawals query
#[derive(Debug)]
pub struct BeaconWithdrawalsQueryBuilder<'a> {
    client: &'a EtherscanClient,
    address: String,
    start_block: Option<u64>,
    end_block: Option<u64>,
    pagination: Pagination,
}

impl<'a> BeaconWithdrawalsQueryBuilder<'a> {
    fn new(client: &'a EtherscanClient, address: &str) -> Self {
        Self {
            client,
            address: address.to_string(),
            start_block: None,
            end_block: None,
            pagination: Pagination::new(),
        }
    }

    /// Set the starting block
    pub fn start_block(mut self, block: u64) -> Self {
        self.start_block = Some(block);
        self
    }

    /// Set the ending block
    pub fn end_block(mut self, block: u64) -> Self {
        self.end_block = Some(block);
        self
    }

    /// Set block range
    pub fn block_range(mut self, start: u64, end: u64) -> Self {
        self.start_block = Some(start);
        self.end_block = Some(end);
        self
    }

    /// Set the page number
    pub fn page(mut self, page: u32) -> Self {
        self.pagination = self.pagination.page(page);
        self
    }

    /// Set the number of results per page
    pub fn offset(mut self, offset: u32) -> Self {
        self.pagination = self.pagination.offset(offset);
        self
    }

    /// Set sort order
    pub fn sort(mut self, sort: Sort) -> Self {
        self.pagination = self.pagination.sort(sort);
        self
    }

    // Getter methods for testing

    pub fn get_address(&self) -> &str {
        &self.address
    }


    pub fn get_start_block(&self) -> Option<u64> {
        self.start_block
    }


    pub fn get_end_block(&self) -> Option<u64> {
        self.end_block
    }


    pub fn get_pagination(&self) -> &Pagination {
        &self.pagination
    }

    /// Execute the query
    pub async fn execute(self) -> Result<Vec<BeaconWithdrawal>> {
        let address = normalize_address(&self.address)?;
        let mut params = vec![("address", address)];

        // Add block range if specified
        if let Some(start) = self.start_block {
            params.push(("startblock", start.to_string()));
        }
        if let Some(end) = self.end_block {
            params.push(("endblock", end.to_string()));
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

        self.client.get("account", "beaconwithdrawal", &params_ref).await
    }
}

/// Builder for historical balance query
#[derive(Debug)]
pub struct HistoricalBalanceQueryBuilder<'a> {
    client: &'a EtherscanClient,
    address: String,
    block_number: Option<u64>,
}

impl<'a> HistoricalBalanceQueryBuilder<'a> {
    fn new(client: &'a EtherscanClient, address: &str) -> Self {
        Self {
            client,
            address: address.to_string(),
            block_number: None,
        }
    }

    /// Set the specific block number to query balance at
    pub fn at_block(mut self, block_number: u64) -> Self {
        self.block_number = Some(block_number);
        self
    }

    // Getter methods for testing

    pub fn get_address(&self) -> &str {
        &self.address
    }


    pub fn get_block_number(&self) -> Option<u64> {
        self.block_number
    }

    /// Execute the query
    pub async fn execute(self) -> Result<Balance> {
        let address = normalize_address(&self.address)?;
        let block_tag = self.block_number
            .map(|n| n.to_string())
            .unwrap_or_else(|| "latest".to_string());

        let params = [
            ("address", address.as_str()),
            ("tag", block_tag.as_str()),
        ];

        self.client.get("account", "balance", &params).await
    }
}

