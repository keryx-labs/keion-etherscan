use crate::{
    error::validation::normalize_tx_hash,
    models::{ContractExecutionStatus, TransactionReceiptStatus, TransactionStatus, TxHash},
    EtherscanClient, Result,
};

/// Transaction-related API endpoints
#[derive(Debug)]
pub struct Transactions<'a> {
    client: &'a EtherscanClient,
}

impl<'a> Transactions<'a> {
    pub fn new(client: &'a EtherscanClient) -> Self {
        Self { client }
    }

    /// Get internal reference to client for query builders
    pub(crate) fn client(&self) -> &'a EtherscanClient {
        self.client
    }

    /// Check the execution status of a contract transaction
    /// Returns whether the contract execution succeeded or failed
    ///
    /// # Arguments
    /// * `tx_hash` - The transaction hash to check
    ///
    /// # Example
    /// ```rust,no_run
    /// use keion_etherscan::EtherscanClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> keion_etherscan::Result<()> {
    ///     let client = EtherscanClient::new("YOUR_API_KEY")?;
    ///     let status = client.transactions()
    ///         .get_execution_status("0x15f8e5ea1079d9a0bb04a4c58ae5fe7654b5b2b4463375ff7ffb490aa0032f3a")
    ///         .await?;
    ///     
    ///     if status.is_successful() {
    ///         println!("Transaction executed successfully");
    ///     } else {
    ///         println!("Transaction failed: {:?}", status.error_message());
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_execution_status<S: AsRef<str>>(
        &self,
        tx_hash: S,
    ) -> Result<ContractExecutionStatus> {
        let tx_hash = normalize_tx_hash(tx_hash.as_ref())?;

        let params = [("txhash", tx_hash.as_str())];

        self.client.get("transaction", "getstatus", &params).await
    }

    /// Check the receipt status of a transaction (post-byzantium fork)
    /// Returns whether the transaction succeeded or failed
    ///
    /// # Arguments
    /// * `tx_hash` - The transaction hash to check
    ///
    /// # Example
    /// ```rust,no_run
    /// use keion_etherscan::EtherscanClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> keion_etherscan::Result<()> {
    ///     let client = EtherscanClient::new("YOUR_API_KEY")?;
    ///     let status = client.transactions()
    ///         .get_receipt_status("0x513c1ba0bebf66436b5fed86ab668452b7805593c05073eb2d51d3a52f480a76")
    ///         .await?;
    ///     
    ///     if status.is_successful() {
    ///         println!("Transaction successful");
    ///     } else {
    ///         println!("Transaction failed");
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_receipt_status<S: AsRef<str>>(
        &self,
        tx_hash: S,
    ) -> Result<TransactionReceiptStatus> {
        let tx_hash = normalize_tx_hash(tx_hash.as_ref())?;

        let params = [("txhash", tx_hash.as_str())];

        self.client
            .get("transaction", "gettxreceiptstatus", &params)
            .await
    }

    /// Get comprehensive transaction status by checking both execution and receipt status
    /// This method combines both status checks for a complete picture
    ///
    /// # Arguments
    /// * `tx_hash` - The transaction hash to check
    ///
    /// # Example
    /// ```rust,no_run
    /// use keion_etherscan::EtherscanClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> keion_etherscan::Result<()> {
    ///     let client = EtherscanClient::new("YOUR_API_KEY")?;
    ///     let status = client.transactions()
    ///         .get_comprehensive_status("0x513c1ba0bebf66436b5fed86ab668452b7805593c05073eb2d51d3a52f480a76")
    ///         .await?;
    ///     
    ///     match status.is_successful() {
    ///         Some(true) => println!("Transaction successful"),
    ///         Some(false) => println!("Transaction failed: {}", status.status_description()),
    ///         None => println!("Status unknown"),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_comprehensive_status<S: AsRef<str>>(
        &self,
        tx_hash: S,
    ) -> Result<TransactionStatus> {
        let tx_hash_str = tx_hash.as_ref();
        let tx_hash = TxHash::new(tx_hash_str);
        let mut status = TransactionStatus::new(tx_hash.clone());

        // Try to get contract execution status (always available)
        match self.get_execution_status(tx_hash_str).await {
            Ok(execution_status) => status.contract_execution = Some(execution_status),
            Err(_) => {
                // Ignore errors, some transactions might not have execution status
            }
        }

        // Try to get receipt status (only for post-Byzantium transactions)
        match self.get_receipt_status(tx_hash_str).await {
            Ok(receipt_status) => status.receipt_status = Some(receipt_status),
            Err(_) => {
                // Ignore errors, pre-Byzantium transactions don't have receipt status
            }
        }

        Ok(status)
    }

    /// Quick check if a transaction succeeded
    /// Returns true if successful, false if failed, None if status cannot be determined
    ///
    /// # Arguments
    /// * `tx_hash` - The transaction hash to check
    ///
    /// # Example
    /// ```rust,no_run
    /// use keion_etherscan::EtherscanClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> keion_etherscan::Result<()> {
    ///     let client = EtherscanClient::new("YOUR_API_KEY")?;
    ///     match client.transactions()
    ///         .is_transaction_successful("0x513c1ba0bebf66436b5fed86ab668452b7805593c05073eb2d51d3a52f480a76")
    ///         .await? {
    ///         Some(true) => println!("Success!"),
    ///         Some(false) => println!("Failed!"),
    ///         None => println!("Unknown status"),
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn is_transaction_successful<S: AsRef<str>>(
        &self,
        tx_hash: S,
    ) -> Result<Option<bool>> {
        let status = self.get_comprehensive_status(tx_hash).await?;
        Ok(status.is_successful())
    }

    /// Create a batch status checker for multiple transactions
    pub fn batch_status(&self) -> TransactionStatusBuilder<'a> {
        TransactionStatusBuilder::new(self.client)
    }
}

/// Builder for checking multiple transaction statuses efficiently
#[derive(Debug)]
pub struct TransactionStatusBuilder<'a> {
    client: &'a EtherscanClient,
    tx_hashes: Vec<TxHash>,
    check_execution: bool,
    check_receipt: bool,
}

impl<'a> TransactionStatusBuilder<'a> {
    fn new(client: &'a EtherscanClient) -> Self {
        Self {
            client,
            tx_hashes: Vec::new(),
            check_execution: true,
            check_receipt: true,
        }
    }

    /// Add a transaction hash to check
    pub fn transaction<S: AsRef<str>>(mut self, tx_hash: S) -> Self {
        self.tx_hashes.push(TxHash::new(tx_hash.as_ref()));
        self
    }

    /// Add multiple transaction hashes
    pub fn transactions<S: AsRef<str>>(mut self, tx_hashes: &[S]) -> Self {
        for hash in tx_hashes {
            self.tx_hashes.push(TxHash::new(hash.as_ref()));
        }
        self
    }

    /// Only check contract execution status
    pub fn execution_only(mut self) -> Self {
        self.check_execution = true;
        self.check_receipt = false;
        self
    }

    /// Only check receipt status
    pub fn receipt_only(mut self) -> Self {
        self.check_execution = false;
        self.check_receipt = true;
        self
    }

    /// Get reference to transaction hashes for testing
    pub fn get_tx_hashes(&self) -> &[TxHash] {
        &self.tx_hashes
    }

    /// Get execution check flag for testing
    pub fn get_check_execution(&self) -> bool {
        self.check_execution
    }

    /// Get receipt check flag for testing
    pub fn get_check_receipt(&self) -> bool {
        self.check_receipt
    }

    /// Execute all status checks
    pub async fn execute(self) -> Result<Vec<TransactionStatus>> {
        let mut results = Vec::new();

        for tx_hash in self.tx_hashes {
            let mut status = TransactionStatus::new(tx_hash.clone());

            if self.check_execution {
                if let Ok(execution) = self
                    .client
                    .get::<ContractExecutionStatus>(
                        "transaction",
                        "getstatus",
                        &[("txhash", tx_hash.as_str())],
                    )
                    .await
                {
                    status.contract_execution = Some(execution);
                }
            }

            if self.check_receipt {
                if let Ok(receipt) = self
                    .client
                    .get::<TransactionReceiptStatus>(
                        "transaction",
                        "gettxreceiptstatus",
                        &[("txhash", tx_hash.as_str())],
                    )
                    .await
                {
                    status.receipt_status = Some(receipt);
                }
            }

            results.push(status);
        }

        Ok(results)
    }
}
