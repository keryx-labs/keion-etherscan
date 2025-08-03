use crate::models::{Address, BigNumber, BlockchainData, HexNumber, StringNumber, TxHash};
use serde::{Deserialize, Serialize};

/// Standard Ethereum transaction
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    /// Block number containing this transaction
    #[serde(rename = "blockNumber")]
    pub block_number: StringNumber,

    /// Block hash containing this transaction
    #[serde(rename = "blockHash")]
    pub block_hash: String,

    /// Transaction index in the block
    #[serde(rename = "transactionIndex")]
    pub transaction_index: StringNumber,

    /// Transaction hash
    pub hash: TxHash,

    /// Nonce of the transaction
    pub nonce: StringNumber,

    /// From address
    pub from: Address,

    /// To address (can be empty for contract creation)
    pub to: Option<Address>,

    /// Value transferred in wei
    pub value: BigNumber,

    /// Gas limit
    pub gas: StringNumber,

    /// Gas price in wei
    #[serde(rename = "gasPrice")]
    pub gas_price: BigNumber,

    /// Gas used by the transaction
    #[serde(rename = "gasUsed")]
    pub gas_used: StringNumber,

    /// Cumulative gas used in the block up to this transaction
    #[serde(rename = "cumulativeGasUsed")]
    pub cumulative_gas_used: StringNumber,

    /// Input data
    pub input: String,

    /// Transaction timestamp
    #[serde(rename = "timeStamp")]
    pub timestamp: StringNumber,

    /// Method ID (first 4 bytes of input)
    #[serde(rename = "methodId")]
    pub method_id: Option<String>,

    /// Function name if available
    #[serde(rename = "functionName")]
    pub function_name: Option<String>,

    /// Transaction receipt status (1 = success, 0 = failed)
    #[serde(rename = "txreceipt_status")]
    pub receipt_status: Option<StringNumber>,

    /// Confirmations
    pub confirmations: Option<StringNumber>,

    /// Is error flag
    #[serde(rename = "isError")]
    pub is_error: Option<StringNumber>,
}

impl Transaction {
    /// Get block number as u64
    pub fn block(&self) -> u64 {
        self.block_number.value()
    }

    /// Get transaction index as u64
    pub fn index(&self) -> u64 {
        self.transaction_index.value()
    }

    /// Get nonce as u64
    pub fn nonce_value(&self) -> u64 {
        self.nonce.value()
    }

    /// Get value in ETH
    pub fn value_eth(&self) -> Option<f64> {
        self.value.as_u128().map(|wei| wei as f64 / 1e18)
    }

    /// Get gas limit as u64
    pub fn gas_limit(&self) -> u64 {
        self.gas.value()
    }

    /// Get gas used as u64
    pub fn gas_used_amount(&self) -> u64 {
        self.gas_used.value()
    }

    /// Get gas price in gwei
    pub fn gas_price_gwei(&self) -> Option<f64> {
        self.gas_price.as_u128().map(|wei| wei as f64 / 1e9)
    }

    /// Get transaction fee in ETH
    pub fn fee_eth(&self) -> Option<f64> {
        let gas_used = self.gas_used_amount();
        let gas_price = self.gas_price.as_u128()?;
        let fee_wei = gas_used as u128 * gas_price;
        Some(fee_wei as f64 / 1e18)
    }

    /// Check if transaction was successful
    pub fn is_successful(&self) -> bool {
        self.receipt_status
            .as_ref()
            .map(|status| status.value() == 1)
            .unwrap_or(true) // Default to true if status not available
    }

    /// Check if transaction had an error
    pub fn has_error(&self) -> bool {
        self.is_error
            .as_ref()
            .map(|error| error.value() == 1)
            .unwrap_or(false)
    }

    /// Get confirmations count
    pub fn confirmation_count(&self) -> Option<u64> {
        self.confirmations.as_ref().map(|c| c.value())
    }

    /// Check if this is a contract creation transaction
    pub fn is_contract_creation(&self) -> bool {
        self.to.is_none() || self.to.as_ref().map(|addr| addr.is_zero()).unwrap_or(false)
    }

    /// Get method signature from input data
    pub fn method_signature(&self) -> Option<&str> {
        if self.input.len() >= 10 && self.input.starts_with("0x") {
            Some(&self.input[0..10])
        } else {
            None
        }
    }
}

impl BlockchainData for Transaction {
    fn block_number(&self) -> Option<u64> {
        Some(self.block())
    }

    fn timestamp(&self) -> Option<u64> {
        Some(self.timestamp.value())
    }
}

/// Internal transaction (trace)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InternalTransaction {
    /// Block number
    #[serde(rename = "blockNumber")]
    pub block_number: StringNumber,

    /// Transaction hash of the parent transaction
    pub hash: TxHash,

    /// From address
    pub from: Address,

    /// To address
    pub to: Option<Address>,

    /// Value transferred in wei
    pub value: BigNumber,

    /// Contract address if this created a contract
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<Address>,

    /// Input data
    pub input: String,

    /// Type of internal transaction
    #[serde(rename = "type")]
    pub transaction_type: String,

    /// Gas limit
    pub gas: StringNumber,

    /// Gas used
    #[serde(rename = "gasUsed")]
    pub gas_used: StringNumber,

    /// Trace ID
    #[serde(rename = "traceId")]
    pub trace_id: String,

    /// Is error flag
    #[serde(rename = "isError")]
    pub is_error: StringNumber,

    /// Error code if any
    #[serde(rename = "errCode")]
    pub error_code: Option<String>,

    /// Timestamp
    #[serde(rename = "timeStamp")]
    pub timestamp: StringNumber,
}

impl InternalTransaction {
    /// Get block number as u64
    pub fn block(&self) -> u64 {
        self.block_number.value()
    }

    /// Get value in ETH
    pub fn value_eth(&self) -> Option<f64> {
        self.value.as_u128().map(|wei| wei as f64 / 1e18)
    }

    /// Check if this internal transaction had an error
    pub fn has_error(&self) -> bool {
        self.is_error.value() == 1
    }

    /// Check if this is a contract creation
    pub fn is_contract_creation(&self) -> bool {
        self.transaction_type == "create" || self.contract_address.is_some()
    }
}

impl BlockchainData for InternalTransaction {
    fn block_number(&self) -> Option<u64> {
        Some(self.block())
    }

    fn timestamp(&self) -> Option<u64> {
        Some(self.timestamp.value())
    }
}

/// Token transfer (ERC-20, ERC-721, ERC-1155)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenTransfer {
    /// Block number
    #[serde(rename = "blockNumber")]
    pub block_number: StringNumber,

    /// Block hash
    #[serde(rename = "blockHash")]
    pub block_hash: String,

    /// Transaction hash
    pub hash: TxHash,

    /// Transaction index
    #[serde(rename = "transactionIndex")]
    pub transaction_index: StringNumber,

    /// From address
    pub from: Address,

    /// To address
    pub to: Address,

    /// Token contract address
    #[serde(rename = "contractAddress")]
    pub contract_address: Address,

    /// Token value/amount transferred
    pub value: BigNumber,

    /// Token name
    #[serde(rename = "tokenName")]
    pub token_name: String,

    /// Token symbol
    #[serde(rename = "tokenSymbol")]
    pub token_symbol: String,

    /// Token decimals
    #[serde(rename = "tokenDecimal")]
    pub token_decimal: StringNumber,

    /// Gas price
    #[serde(rename = "gasPrice")]
    pub gas_price: BigNumber,

    /// Gas used
    #[serde(rename = "gasUsed")]
    pub gas_used: StringNumber,

    /// Timestamp
    #[serde(rename = "timeStamp")]
    pub timestamp: StringNumber,

    /// Log index
    #[serde(rename = "logIndex")]
    pub log_index: StringNumber,

    /// Token ID (for NFTs)
    #[serde(rename = "tokenID")]
    pub token_id: Option<BigNumber>,

    /// Confirmations
    pub confirmations: Option<StringNumber>,
}

impl TokenTransfer {
    /// Get block number as u64
    pub fn block(&self) -> u64 {
        self.block_number.value()
    }

    /// Get token decimals as u64
    pub fn decimals(&self) -> u64 {
        self.token_decimal.value()
    }

    /// Get token amount as decimal value
    pub fn decimal_value(&self) -> Option<f64> {
        let value = self.value.as_u128()?;
        let decimals = self.decimals();
        Some(value as f64 / 10_f64.powi(decimals as i32))
    }

    /// Check if this is an NFT transfer (typically has token_id)
    pub fn is_nft(&self) -> bool {
        self.token_id.is_some() || self.decimals() == 0
    }

    /// Get token ID for NFTs
    pub fn nft_token_id(&self) -> Option<&str> {
        self.token_id.as_ref().map(|id| id.as_str())
    }

    /// Get gas price in gwei
    pub fn gas_price_gwei(&self) -> Option<f64> {
        self.gas_price.as_u128().map(|wei| wei as f64 / 1e9)
    }
}

impl BlockchainData for TokenTransfer {
    fn block_number(&self) -> Option<u64> {
        Some(self.block())
    }

    fn timestamp(&self) -> Option<u64> {
        Some(self.timestamp.value())
    }
}

/// Transaction receipt with logs and status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionReceipt {
    /// Block hash
    #[serde(rename = "blockHash")]
    pub block_hash: String,

    /// Block number
    #[serde(rename = "blockNumber")]
    pub block_number: HexNumber,

    /// Contract address if created
    #[serde(rename = "contractAddress")]
    pub contract_address: Option<Address>,

    /// Cumulative gas used
    #[serde(rename = "cumulativeGasUsed")]
    pub cumulative_gas_used: HexNumber,

    /// From address
    pub from: Address,

    /// Gas used
    #[serde(rename = "gasUsed")]
    pub gas_used: HexNumber,

    /// Transaction logs
    pub logs: Vec<TransactionLog>,

    /// Logs bloom filter
    #[serde(rename = "logsBloom")]
    pub logs_bloom: String,

    /// Status (1 = success, 0 = failed)
    pub status: HexNumber,

    /// To address
    pub to: Option<Address>,

    /// Transaction hash
    #[serde(rename = "transactionHash")]
    pub transaction_hash: TxHash,

    /// Transaction index
    #[serde(rename = "transactionIndex")]
    pub transaction_index: HexNumber,
}

impl TransactionReceipt {
    /// Check if transaction was successful
    pub fn is_successful(&self) -> bool {
        self.status.value() == 1
    }

    /// Get block number as u64
    pub fn block(&self) -> u64 {
        self.block_number.value()
    }

    /// Get gas used as u64
    pub fn gas_used_amount(&self) -> u64 {
        self.gas_used.value()
    }
}

/// Transaction log entry
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransactionLog {
    /// Contract address that emitted the log
    pub address: Address,

    /// Log topics
    pub topics: Vec<String>,

    /// Log data
    pub data: String,

    /// Block number
    #[serde(rename = "blockNumber")]
    pub block_number: HexNumber,

    /// Transaction hash
    #[serde(rename = "transactionHash")]
    pub transaction_hash: TxHash,

    /// Transaction index
    #[serde(rename = "transactionIndex")]
    pub transaction_index: HexNumber,

    /// Block hash
    #[serde(rename = "blockHash")]
    pub block_hash: String,

    /// Log index
    #[serde(rename = "logIndex")]
    pub log_index: HexNumber,

    /// Removed flag
    pub removed: bool,
}

impl TransactionLog {
    /// Get the event signature (first topic)
    pub fn event_signature(&self) -> Option<&str> {
        self.topics.first().map(|s| s.as_str())
    }

    /// Get block number as u64
    pub fn block(&self) -> u64 {
        self.block_number.value()
    }
}
