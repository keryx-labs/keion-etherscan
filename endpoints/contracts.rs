use crate::{
    error::validation::normalize_address,
    models::{
        CodeFormat, ContractAbi, ContractCreation, ContractSource, LibraryLink,
        OptimizationSettings, ProxyVerificationStatus, VerificationRequest, VerificationStatus,
    },
    EtherscanClient, EtherscanError, Result,
};
use std::collections::HashMap;

/// Contract-related API endpoints
#[derive(Debug)]
pub struct Contracts<'a> {
    client: &'a EtherscanClient,
}

impl<'a> Contracts<'a> {
    pub fn new(client: &'a EtherscanClient) -> Self {
        Self { client }
    }

    /// Get Contract ABI for Verified Contract Source Codes
    ///
    /// # Arguments
    /// * `address` - The contract address to get ABI for
    ///
    /// # Example
    /// ```rust,no_run
    /// use keion_etherscan::EtherscanClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> keion_etherscan::Result<()> {
    ///     let client = EtherscanClient::new("YOUR_API_KEY")?;
    ///     let abi = client.contracts()
    ///         .get_abi("0xdAC17F958D2ee523a2206206994597C13D831ec7")
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_abi<S: AsRef<str>>(&self, address: S) -> Result<ContractAbi> {
        let address = normalize_address(address.as_ref())?;

        let params = [("address", address.as_str())];

        self.client.get("contract", "getabi", &params).await
    }

    /// Get Contract Source Code for Verified Contract Source Codes
    ///
    /// # Arguments
    /// * `address` - The contract address to get source code for
    ///
    /// # Example
    /// ```rust,no_run
    /// use keion_etherscan::EtherscanClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> keion_etherscan::Result<()> {
    ///     let client = EtherscanClient::new("YOUR_API_KEY")?;
    ///     let source = client.contracts()
    ///         .get_source_code("0xdAC17F958D2ee523a2206206994597C13D831ec7")
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_source_code<S: AsRef<str>>(&self, address: S) -> Result<Vec<ContractSource>> {
        let address = normalize_address(address.as_ref())?;

        let params = [("address", address.as_str())];

        self.client.get("contract", "getsourcecode", &params).await
    }

    /// Get Contract Creator and Creation Tx Hash
    ///
    /// # Arguments
    /// * `addresses` - List of contract addresses (up to 5 addresses)
    ///
    /// # Example
    /// ```rust,no_run
    /// use keion_etherscan::EtherscanClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> keion_etherscan::Result<()> {
    ///     let client = EtherscanClient::new("YOUR_API_KEY")?;
    ///     let creators = client.contracts()
    ///         .get_contract_creation(&[
    ///             "0xdAC17F958D2ee523a2206206994597C13D831ec7",
    ///             "0xA0b86a33E6411b7A0a6acc95b0e8fd65B7b1b6c8"
    ///         ])
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_contract_creation<S: AsRef<str>>(
        &self,
        addresses: &[S],
    ) -> Result<Vec<ContractCreation>> {
        if addresses.is_empty() {
            return Err(EtherscanError::InvalidParams(
                "At least one address required".to_string(),
            ));
        }

        if addresses.len() > 5 {
            return Err(EtherscanError::InvalidParams(
                "Maximum 5 addresses allowed".to_string(),
            ));
        }

        // Validate and normalize all addresses
        let normalized: Result<Vec<String>> = addresses
            .iter()
            .map(|addr| normalize_address(addr.as_ref()))
            .collect();
        let normalized = normalized?;

        let address_list = normalized.join(",");
        let params = [("contractaddresses", address_list.as_str())];

        self.client
            .get("contract", "getcontractcreation", &params)
            .await
    }

    /// Create a Solidity verification builder
    ///
    /// # Arguments
    /// * `address` - The contract address to verify
    ///
    /// # Example
    /// ```rust,no_run
    /// use keion_etherscan::{EtherscanClient, CodeFormat};
    ///
    /// #[tokio::main]
    /// async fn main() -> keion_etherscan::Result<()> {
    ///     let client = EtherscanClient::new("YOUR_API_KEY")?;
    ///     let request = client.contracts()
    ///         .verify_solidity("0x742d35cc6634c0532925a3b8d19389c4d5e1e4a6")
    ///         .source_code("pragma solidity ^0.8.0; contract Test {}")
    ///         .contract_name("Test")
    ///         .compiler_version("v0.8.24+commit.e11b9ed9")
    ///         .optimization(true, 200)
    ///         .code_format(CodeFormat::SoliditySingleFile)
    ///         .submit()
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn verify_solidity<S: AsRef<str>>(&self, address: S) -> SolidityVerificationBuilder<'a> {
        SolidityVerificationBuilder::new(self.client, address.as_ref())
    }

    /// Create a Vyper verification builder
    ///
    /// # Arguments
    /// * `address` - The contract address to verify
    pub fn verify_vyper<S: AsRef<str>>(&self, address: S) -> VyperVerificationBuilder<'a> {
        VyperVerificationBuilder::new(self.client, address.as_ref())
    }

    /// Check Source Code Verification Status
    ///
    /// # Arguments
    /// * `guid` - The GUID returned from a verification request
    ///
    /// # Example
    /// ```rust,no_run
    /// use keion_etherscan::EtherscanClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> keion_etherscan::Result<()> {
    ///     let client = EtherscanClient::new("YOUR_API_KEY")?;
    ///     let status = client.contracts()
    ///         .check_verification_status("ezq878u486pzijgvynpjq")
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn check_verification_status<S: AsRef<str>>(
        &self,
        guid: S,
    ) -> Result<VerificationStatus> {
        let params = [("guid", guid.as_ref())];

        self.client
            .get("contract", "checkverifystatus", &params)
            .await
    }

    /// Create a proxy verification builder
    ///
    /// # Arguments
    /// * `address` - The proxy contract address to verify
    pub fn verify_proxy<S: AsRef<str>>(&self, address: S) -> ProxyVerificationBuilder<'a> {
        ProxyVerificationBuilder::new(self.client, address.as_ref())
    }

    /// Check Proxy Verification Status
    ///
    /// # Arguments
    /// * `guid` - The GUID returned from a proxy verification request
    pub async fn check_proxy_verification_status<S: AsRef<str>>(
        &self,
        guid: S,
    ) -> Result<ProxyVerificationStatus> {
        let params = [("guid", guid.as_ref())];

        self.client
            .get("contract", "checkproxyverification", &params)
            .await
    }
}

/// Builder for Solidity contract verification
#[derive(Debug)]
pub struct SolidityVerificationBuilder<'a> {
    client: &'a EtherscanClient,
    address: String,
    source_code: Option<String>,
    contract_name: Option<String>,
    compiler_version: Option<String>,
    optimization_settings: OptimizationSettings,
    constructor_arguments: Option<String>,
    code_format: CodeFormat,
    libraries: Vec<LibraryLink>,
    license_type: Option<String>,
    evm_version: Option<String>,
}

impl<'a> SolidityVerificationBuilder<'a> {
    fn new(client: &'a EtherscanClient, address: &str) -> Self {
        Self {
            client,
            address: address.to_string(),
            source_code: None,
            contract_name: None,
            compiler_version: None,
            optimization_settings: OptimizationSettings::disabled(),
            constructor_arguments: None,
            code_format: CodeFormat::SoliditySingleFile,
            libraries: Vec::new(),
            license_type: None,
            evm_version: None,
        }
    }

    /// Set the source code for verification
    pub fn source_code<S: Into<String>>(mut self, source_code: S) -> Self {
        self.source_code = Some(source_code.into());
        self
    }

    /// Set the contract name
    pub fn contract_name<S: Into<String>>(mut self, name: S) -> Self {
        self.contract_name = Some(name.into());
        self
    }

    /// Set the compiler version
    pub fn compiler_version<S: Into<String>>(mut self, version: S) -> Self {
        self.compiler_version = Some(version.into());
        self
    }

    /// Set optimization settings
    pub fn optimization(mut self, enabled: bool, runs: u32) -> Self {
        self.optimization_settings = if enabled {
            OptimizationSettings::enabled(runs)
        } else {
            OptimizationSettings::disabled()
        };
        self
    }

    /// Set optimization settings using OptimizationSettings struct
    pub fn optimization_settings(mut self, settings: OptimizationSettings) -> Self {
        self.optimization_settings = settings;
        self
    }

    /// Set constructor arguments
    pub fn constructor_arguments<S: Into<String>>(mut self, args: S) -> Self {
        self.constructor_arguments = Some(args.into());
        self
    }

    /// Set the code format
    pub fn code_format(mut self, format: CodeFormat) -> Self {
        self.code_format = format;
        self
    }

    /// Add a library link
    pub fn library<S: Into<String>>(mut self, name: S, address: S) -> Self {
        self.libraries.push(LibraryLink::new(name, address));
        self
    }

    /// Add multiple library links
    pub fn libraries(mut self, libraries: Vec<LibraryLink>) -> Self {
        self.libraries.extend(libraries);
        self
    }

    /// Set the license type
    pub fn license_type<S: Into<String>>(mut self, license: S) -> Self {
        self.license_type = Some(license.into());
        self
    }

    /// Set the EVM version
    pub fn evm_version<S: Into<String>>(mut self, version: S) -> Self {
        self.evm_version = Some(version.into());
        self
    }

    /// Get the source code (for testing)
    pub fn get_source_code(&self) -> &Option<String> {
        &self.source_code
    }

    /// Get the contract name (for testing)
    pub fn get_contract_name(&self) -> &Option<String> {
        &self.contract_name
    }

    /// Get the compiler version (for testing)
    pub fn get_compiler_version(&self) -> &Option<String> {
        &self.compiler_version
    }

    /// Get the optimization settings (for testing)
    pub fn get_optimization_settings(&self) -> &OptimizationSettings {
        &self.optimization_settings
    }

    /// Get the constructor arguments (for testing)
    pub fn get_constructor_arguments(&self) -> &Option<String> {
        &self.constructor_arguments
    }

    /// Get the code format (for testing)
    pub fn get_code_format(&self) -> CodeFormat {
        self.code_format
    }

    /// Get the libraries (for testing)
    pub fn get_libraries(&self) -> &[LibraryLink] {
        &self.libraries
    }

    /// Submit the verification request
    pub async fn submit(self) -> Result<VerificationRequest> {
        let address = normalize_address(&self.address)?;

        let source_code = self
            .source_code
            .ok_or_else(|| EtherscanError::InvalidParams("Source code is required".to_string()))?;
        let contract_name = self.contract_name.ok_or_else(|| {
            EtherscanError::InvalidParams("Contract name is required".to_string())
        })?;
        let compiler_version = self.compiler_version.ok_or_else(|| {
            EtherscanError::InvalidParams("Compiler version is required".to_string())
        })?;

        let mut form_data = HashMap::new();
        form_data.insert("contractaddress".to_string(), address);
        form_data.insert("sourceCode".to_string(), source_code);
        form_data.insert("contractname".to_string(), contract_name);
        form_data.insert("compilerversion".to_string(), compiler_version);
        form_data.insert(
            "optimizationUsed".to_string(),
            if self.optimization_settings.enabled {
                "1"
            } else {
                "0"
            }
            .to_string(),
        );

        if self.optimization_settings.enabled {
            form_data.insert(
                "runs".to_string(),
                self.optimization_settings.runs.to_string(),
            );
        }

        if let Some(args) = self.constructor_arguments {
            form_data.insert("constructorArguements".to_string(), args); // Note: Etherscan API uses this typo
        }

        form_data.insert(
            "codeformat".to_string(),
            self.code_format.as_str().to_string(),
        );

        if let Some(license) = self.license_type {
            form_data.insert("licenseType".to_string(), license);
        }

        if let Some(evm_version) = self.evm_version {
            form_data.insert("evmversion".to_string(), evm_version);
        }

        // Add library information
        for (i, library) in self.libraries.iter().enumerate() {
            let name_key = format!("libraryname{}", i + 1);
            let address_key = format!("libraryaddress{}", i + 1);
            form_data.insert(name_key, library.name.clone());
            form_data.insert(address_key, library.address.clone());
        }

        // TODO: Implement POST method in client for verification endpoints
        // For now, return a placeholder to allow compilation
        Err(EtherscanError::InvalidParams(
            "POST method not yet implemented".to_string(),
        ))
    }
}

/// Builder for Vyper contract verification
#[derive(Debug)]
pub struct VyperVerificationBuilder<'a> {
    client: &'a EtherscanClient,
    address: String,
    source_code: Option<String>,
    contract_name: Option<String>,
    compiler_version: Option<String>,
    constructor_arguments: Option<String>,
    optimization_settings: OptimizationSettings,
}

impl<'a> VyperVerificationBuilder<'a> {
    fn new(client: &'a EtherscanClient, address: &str) -> Self {
        Self {
            client,
            address: address.to_string(),
            source_code: None,
            contract_name: None,
            compiler_version: None,
            constructor_arguments: None,
            optimization_settings: OptimizationSettings::disabled(),
        }
    }

    /// Set the source code for verification
    pub fn source_code<S: Into<String>>(mut self, source_code: S) -> Self {
        self.source_code = Some(source_code.into());
        self
    }

    /// Set the contract name
    pub fn contract_name<S: Into<String>>(mut self, name: S) -> Self {
        self.contract_name = Some(name.into());
        self
    }

    /// Set the compiler version
    pub fn compiler_version<S: Into<String>>(mut self, version: S) -> Self {
        self.compiler_version = Some(version.into());
        self
    }

    /// Set constructor arguments
    pub fn constructor_arguments<S: Into<String>>(mut self, args: S) -> Self {
        self.constructor_arguments = Some(args.into());
        self
    }

    /// Set optimization settings
    pub fn optimization(mut self, enabled: bool, runs: u32) -> Self {
        self.optimization_settings = if enabled {
            OptimizationSettings::enabled(runs)
        } else {
            OptimizationSettings::disabled()
        };
        self
    }

    /// Get the source code (for testing)
    pub fn get_source_code(&self) -> &Option<String> {
        &self.source_code
    }

    /// Get the contract name (for testing)
    pub fn get_contract_name(&self) -> &Option<String> {
        &self.contract_name
    }

    /// Get the compiler version (for testing)
    pub fn get_compiler_version(&self) -> &Option<String> {
        &self.compiler_version
    }

    /// Get the optimization settings (for testing)
    pub fn get_optimization_settings(&self) -> &OptimizationSettings {
        &self.optimization_settings
    }

    /// Submit the verification request
    pub async fn submit(self) -> Result<VerificationRequest> {
        let address = normalize_address(&self.address)?;

        let source_code = self
            .source_code
            .ok_or_else(|| EtherscanError::InvalidParams("Source code is required".to_string()))?;
        let contract_name = self.contract_name.ok_or_else(|| {
            EtherscanError::InvalidParams("Contract name is required".to_string())
        })?;
        let compiler_version = self.compiler_version.ok_or_else(|| {
            EtherscanError::InvalidParams("Compiler version is required".to_string())
        })?;

        let mut form_data = HashMap::new();
        form_data.insert("contractaddress".to_string(), address);
        form_data.insert("sourceCode".to_string(), source_code);
        form_data.insert("contractname".to_string(), contract_name);
        form_data.insert("compilerversion".to_string(), compiler_version);
        form_data.insert(
            "optimizationUsed".to_string(),
            if self.optimization_settings.enabled {
                "1"
            } else {
                "0"
            }
            .to_string(),
        );

        if self.optimization_settings.enabled {
            form_data.insert(
                "runs".to_string(),
                self.optimization_settings.runs.to_string(),
            );
        }

        if let Some(args) = self.constructor_arguments {
            form_data.insert("constructorArguements".to_string(), args); // Note: Etherscan API uses this typo
        }

        form_data.insert(
            "codeformat".to_string(),
            CodeFormat::VyperJson.as_str().to_string(),
        );

        // TODO: Implement POST method in client for verification endpoints
        // For now, return a placeholder to allow compilation
        Err(EtherscanError::InvalidParams(
            "POST method not yet implemented".to_string(),
        ))
    }
}

/// Builder for proxy contract verification
#[derive(Debug)]
pub struct ProxyVerificationBuilder<'a> {
    client: &'a EtherscanClient,
    address: String,
    expected_implementation: Option<String>,
}

impl<'a> ProxyVerificationBuilder<'a> {
    fn new(client: &'a EtherscanClient, address: &str) -> Self {
        Self {
            client,
            address: address.to_string(),
            expected_implementation: None,
        }
    }

    /// Set the expected implementation address
    pub fn expected_implementation<S: Into<String>>(mut self, implementation: S) -> Self {
        self.expected_implementation = Some(implementation.into());
        self
    }

    /// Get the expected implementation (for testing)
    pub fn get_expected_implementation(&self) -> &Option<String> {
        &self.expected_implementation
    }

    /// Submit the proxy verification request
    pub async fn submit(self) -> Result<VerificationRequest> {
        let address = normalize_address(&self.address)?;

        let mut form_data = HashMap::new();
        form_data.insert("address".to_string(), address);

        if let Some(implementation) = self.expected_implementation {
            let normalized_impl = normalize_address(&implementation)?;
            form_data.insert("expectedimplementation".to_string(), normalized_impl);
        }

        // TODO: Implement POST method in client for verification endpoints
        // For now, return a placeholder to allow compilation
        Err(EtherscanError::InvalidParams(
            "POST method not yet implemented".to_string(),
        ))
    }
}
