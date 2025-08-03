use crate::models::{Address, StringNumber, TxHash};
use serde::{Deserialize, Serialize};

/// Contract ABI representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractAbi {
    /// JSON string of the ABI
    #[serde(rename = "ABI")]
    pub abi: String,
}

impl ContractAbi {
    /// Parse the ABI JSON string into a JSON value
    pub fn parse_abi(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::from_str(&self.abi)
    }

    /// Check if the ABI is empty
    pub fn is_empty(&self) -> bool {
        self.abi.is_empty() || self.abi == "Contract source code not verified"
    }
}

/// Contract source code information
#[derive(Debug, Clone, Deserialize)]
pub struct ContractSource {
    /// The source code of the contract
    #[serde(rename = "SourceCode")]
    pub source_code: String,

    /// The ABI JSON string
    #[serde(rename = "ABI")]
    pub abi: String,

    /// Name of the contract
    #[serde(rename = "ContractName")]
    pub contract_name: String,

    /// Compiler version used
    #[serde(rename = "CompilerVersion")]
    pub compiler_version: String,

    /// Whether optimization was used (0 or 1)
    #[serde(rename = "OptimizationUsed")]
    pub optimization_used: StringNumber,

    /// Number of optimization runs
    #[serde(rename = "Runs")]
    pub runs: StringNumber,

    /// Constructor arguments
    #[serde(rename = "ConstructorArguments")]
    pub constructor_arguments: String,

    /// EVM version used
    #[serde(rename = "EVMVersion")]
    pub evm_version: String,

    /// Library information
    #[serde(rename = "Library")]
    pub library: String,

    /// License type
    #[serde(rename = "LicenseType")]
    pub license_type: String,

    /// Whether this is a proxy contract (0 or 1)
    #[serde(rename = "Proxy")]
    pub proxy: StringNumber,

    /// Implementation address for proxy contracts
    #[serde(rename = "Implementation")]
    pub implementation: String,

    /// Swarm source hash
    #[serde(rename = "SwarmSource")]
    pub swarm_source: String,
}

impl ContractSource {
    /// Check if optimization was used during compilation
    pub fn is_optimized(&self) -> bool {
        self.optimization_used.value() == 1
    }

    /// Check if this is a proxy contract
    pub fn is_proxy(&self) -> bool {
        self.proxy.value() == 1
    }

    /// Get the number of optimization runs
    pub fn optimization_runs(&self) -> u64 {
        self.runs.value()
    }

    /// Check if the contract source is verified
    pub fn is_verified(&self) -> bool {
        !self.source_code.is_empty() && self.source_code != "Contract source code not verified"
    }

    /// Parse the ABI JSON string
    pub fn parse_abi(&self) -> serde_json::Result<serde_json::Value> {
        serde_json::from_str(&self.abi)
    }
}

/// Contract creation information
#[derive(Debug, Clone, Deserialize)]
pub struct ContractCreation {
    /// The contract address
    #[serde(rename = "contractAddress")]
    pub contract_address: Address,

    /// Address of the account that created the contract
    #[serde(rename = "contractCreator")]
    pub contract_creator: Address,

    /// Transaction hash of the creation transaction
    #[serde(rename = "txHash")]
    pub tx_hash: TxHash,
}

/// Verification status response
#[derive(Debug, Clone, Deserialize)]
pub struct VerificationStatus {
    /// Status message from Etherscan
    pub status: String,
}

impl VerificationStatus {
    /// Check if verification was successful
    pub fn is_verified(&self) -> bool {
        self.status.starts_with("Pass")
    }

    /// Check if verification failed
    pub fn is_failed(&self) -> bool {
        self.status.starts_with("Fail")
    }

    /// Check if verification is still pending
    pub fn is_pending(&self) -> bool {
        self.status.contains("Pending")
    }
}

/// Proxy verification status response
#[derive(Debug, Clone, Deserialize)]
pub struct ProxyVerificationStatus {
    /// Result message from Etherscan
    pub result: String,
}

impl ProxyVerificationStatus {
    /// Check if proxy verification was successful
    pub fn is_verified(&self) -> bool {
        self.result.contains("successfully updated")
    }
}

/// Verification request response containing GUID for tracking
#[derive(Debug, Clone, Deserialize)]
pub struct VerificationRequest {
    /// GUID for tracking verification status
    pub guid: String,
}

/// Code format types for verification
#[derive(Debug, Clone, Copy)]
pub enum CodeFormat {
    /// Single Solidity file
    SoliditySingleFile,
    /// Solidity standard JSON input
    SolidityStandardJsonInput,
    /// Vyper JSON format
    VyperJson,
}

impl CodeFormat {
    /// Get the string representation for API calls
    pub fn as_str(&self) -> &'static str {
        match self {
            CodeFormat::SoliditySingleFile => "solidity-single-file",
            CodeFormat::SolidityStandardJsonInput => "solidity-standard-json-input",
            CodeFormat::VyperJson => "vyper-json",
        }
    }
}

/// Optimization settings for contract verification
#[derive(Debug, Clone)]
pub struct OptimizationSettings {
    /// Whether optimization is enabled
    pub enabled: bool,
    /// Number of optimization runs (if enabled)
    pub runs: u32,
}

impl OptimizationSettings {
    /// Create optimization settings with enabled optimization
    pub fn enabled(runs: u32) -> Self {
        Self {
            enabled: true,
            runs,
        }
    }

    /// Create optimization settings with disabled optimization
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            runs: 0,
        }
    }
}

/// Library linking information for contract verification
#[derive(Debug, Clone)]
pub struct LibraryLink {
    /// Library name
    pub name: String,
    /// Library address
    pub address: String,
}

impl LibraryLink {
    /// Create a new library link
    pub fn new<S: Into<String>>(name: S, address: S) -> Self {
        Self {
            name: name.into(),
            address: address.into(),
        }
    }
}
