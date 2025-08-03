mod common;

use common::{TestConstants, TestUtils};
use keion_etherscan::{
    CodeFormat, EtherscanClient, LibraryLink, Network, OptimizationSettings,
    ProxyVerificationBuilder, SolidityVerificationBuilder, VyperVerificationBuilder,
};

/// Test all contract endpoint builders (non-async tests)
mod builder_tests {
    use super::*;

    #[test]
    fn test_solidity_verification_builder() {
        let client = TestUtils::create_test_client();
        let contracts = client.contracts();

        let builder = contracts
            .verify_solidity(TestUtils::contract_address())
            .source_code("pragma solidity ^0.8.0; contract Test { uint256 public value; }")
            .contract_name("Test")
            .compiler_version("v0.8.24+commit.e11b9ed9")
            .optimization(true, 200)
            .code_format(CodeFormat::SoliditySingleFile)
            .constructor_arguments(
                "0x000000000000000000000000000000000000000000000000000000000000007b",
            )
            .license_type("MIT")
            .evm_version("default");

        // Test builder state
        assert!(builder.get_source_code().is_some());
        assert_eq!(
            builder.get_source_code().as_ref().unwrap(),
            "pragma solidity ^0.8.0; contract Test { uint256 public value; }"
        );
        assert_eq!(builder.get_contract_name().as_ref().unwrap(), "Test");
        assert_eq!(
            builder.get_compiler_version().as_ref().unwrap(),
            "v0.8.24+commit.e11b9ed9"
        );
        assert!(builder.get_optimization_settings().enabled);
        assert_eq!(builder.get_optimization_settings().runs, 200);
        assert_eq!(
            builder.get_constructor_arguments().as_ref().unwrap(),
            "0x000000000000000000000000000000000000000000000000000000000000007b"
        );

        // Test code format
        match builder.get_code_format() {
            CodeFormat::SoliditySingleFile => {}
            _ => panic!("Expected SoliditySingleFile format"),
        }
    }

    #[test]
    fn test_solidity_verification_builder_with_libraries() {
        let client = TestUtils::create_test_client();
        let contracts = client.contracts();

        let builder = contracts
            .verify_solidity(TestUtils::contract_address())
            .source_code("pragma solidity ^0.8.0; import './SafeMath.sol'; contract Test { using SafeMath for uint256; }")
            .contract_name("Test")
            .compiler_version("v0.8.24+commit.e11b9ed9")
            .optimization(false, 0)
            .library("SafeMath", "0x1234567890123456789012345678901234567890")
            .library("Another", "0x0987654321098765432109876543210987654321");

        // Test library state
        let libraries = builder.get_libraries();
        assert_eq!(libraries.len(), 2);
        assert_eq!(libraries[0].name, "SafeMath");
        assert_eq!(
            libraries[0].address,
            "0x1234567890123456789012345678901234567890"
        );
        assert_eq!(libraries[1].name, "Another");
        assert_eq!(
            libraries[1].address,
            "0x0987654321098765432109876543210987654321"
        );

        // Test optimization settings
        assert!(!builder.get_optimization_settings().enabled);
        assert_eq!(builder.get_optimization_settings().runs, 0);
    }

    #[test]
    fn test_solidity_verification_builder_with_multiple_libraries() {
        let client = TestUtils::create_test_client();
        let contracts = client.contracts();

        let libs = vec![
            LibraryLink::new("Library1", "0x1111111111111111111111111111111111111111"),
            LibraryLink::new("Library2", "0x2222222222222222222222222222222222222222"),
        ];

        let builder = contracts
            .verify_solidity(TestUtils::contract_address())
            .source_code("pragma solidity ^0.8.0; contract Test {}")
            .contract_name("Test")
            .compiler_version("v0.8.24+commit.e11b9ed9")
            .libraries(libs);

        let libraries = builder.get_libraries();
        assert_eq!(libraries.len(), 2);
        assert_eq!(libraries[0].name, "Library1");
        assert_eq!(libraries[1].name, "Library2");
    }

    #[test]
    fn test_solidity_verification_builder_optimization_settings() {
        let client = TestUtils::create_test_client();
        let contracts = client.contracts();

        // Test with OptimizationSettings struct
        let opt_settings = OptimizationSettings::enabled(500);
        let builder = contracts
            .verify_solidity(TestUtils::contract_address())
            .source_code("pragma solidity ^0.8.0; contract Test {}")
            .contract_name("Test")
            .compiler_version("v0.8.24+commit.e11b9ed9")
            .optimization_settings(opt_settings);

        assert!(builder.get_optimization_settings().enabled);
        assert_eq!(builder.get_optimization_settings().runs, 500);

        // Test disabled optimization
        let disabled_settings = OptimizationSettings::disabled();
        let builder2 = contracts
            .verify_solidity(TestUtils::contract_address())
            .optimization_settings(disabled_settings);

        assert!(!builder2.get_optimization_settings().enabled);
        assert_eq!(builder2.get_optimization_settings().runs, 0);
    }

    #[test]
    fn test_vyper_verification_builder() {
        let client = TestUtils::create_test_client();
        let contracts = client.contracts();

        let builder = contracts
            .verify_vyper(TestUtils::contract_address())
            .source_code("# @version ^0.3.0\n@external\ndef test() -> uint256:\n    return 42")
            .contract_name("VyperTest")
            .compiler_version("v0.3.10+commit.91361694")
            .optimization(true, 100)
            .constructor_arguments("0x");

        // Test builder state
        assert!(builder.get_source_code().is_some());
        assert_eq!(builder.get_contract_name().as_ref().unwrap(), "VyperTest");
        assert_eq!(
            builder.get_compiler_version().as_ref().unwrap(),
            "v0.3.10+commit.91361694"
        );
        assert!(builder.get_optimization_settings().enabled);
        assert_eq!(builder.get_optimization_settings().runs, 100);
    }

    #[test]
    fn test_proxy_verification_builder() {
        let client = TestUtils::create_test_client();
        let contracts = client.contracts();

        let builder = contracts
            .verify_proxy(TestUtils::contract_address())
            .expected_implementation("0x1234567890123456789012345678901234567890");

        // Test builder state
        assert!(builder.get_expected_implementation().is_some());
        assert_eq!(
            builder.get_expected_implementation().as_ref().unwrap(),
            "0x1234567890123456789012345678901234567890"
        );
    }

    #[test]
    fn test_proxy_verification_builder_without_implementation() {
        let client = TestUtils::create_test_client();
        let contracts = client.contracts();

        let builder = contracts.verify_proxy(TestUtils::contract_address());

        // Test builder state - should be None by default
        assert!(builder.get_expected_implementation().is_none());
    }
}

/// Test parameter validation for contract endpoints
mod parameter_validation_tests {
    use super::*;

    #[test]
    fn test_get_contract_creation_empty_addresses() {
        let client = TestUtils::create_test_client();
        let contracts = client.contracts();

        // This would be tested in async tests, but we can test the validation logic
        let empty_addresses: Vec<&str> = vec![];
        // Note: We can't test async functions here, so we'll test this in async tests
    }

    #[test]
    fn test_get_contract_creation_too_many_addresses() {
        let client = TestUtils::create_test_client();
        let contracts = client.contracts();

        // Create 6 addresses (more than the limit of 5)
        let too_many_addresses = vec![
            "0x1111111111111111111111111111111111111111",
            "0x2222222222222222222222222222222222222222",
            "0x3333333333333333333333333333333333333333",
            "0x4444444444444444444444444444444444444444",
            "0x5555555555555555555555555555555555555555",
            "0x6666666666666666666666666666666666666666", // This exceeds the limit
        ];
        // Note: We can't test async functions here, so we'll test this in async tests
    }

    #[test]
    fn test_code_format_string_representations() {
        assert_eq!(
            CodeFormat::SoliditySingleFile.as_str(),
            "solidity-single-file"
        );
        assert_eq!(
            CodeFormat::SolidityStandardJsonInput.as_str(),
            "solidity-standard-json-input"
        );
        assert_eq!(CodeFormat::VyperJson.as_str(), "vyper-json");
    }

    #[test]
    fn test_optimization_settings_creation() {
        let enabled = OptimizationSettings::enabled(200);
        assert!(enabled.enabled);
        assert_eq!(enabled.runs, 200);

        let disabled = OptimizationSettings::disabled();
        assert!(!disabled.enabled);
        assert_eq!(disabled.runs, 0);
    }

    #[test]
    fn test_library_link_creation() {
        let link = LibraryLink::new("TestLib", "0x1234567890123456789012345678901234567890");
        assert_eq!(link.name, "TestLib");
        assert_eq!(link.address, "0x1234567890123456789012345678901234567890");

        // Test with different string types
        let link2 = LibraryLink::new(
            String::from("AnotherLib"),
            String::from("0x0987654321098765432109876543210987654321"),
        );
        assert_eq!(link2.name, "AnotherLib");
        assert_eq!(link2.address, "0x0987654321098765432109876543210987654321");
    }
}

/// Test edge cases and boundary conditions
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_solidity_builder_minimal_configuration() {
        let client = TestUtils::create_test_client();
        let contracts = client.contracts();

        let builder = contracts.verify_solidity(TestUtils::contract_address());

        // Test default values
        assert!(builder.get_source_code().is_none());
        assert!(builder.get_contract_name().is_none());
        assert!(builder.get_compiler_version().is_none());
        assert!(!builder.get_optimization_settings().enabled);
        assert_eq!(builder.get_optimization_settings().runs, 0);
        assert!(builder.get_constructor_arguments().is_none());
        assert_eq!(builder.get_libraries().len(), 0);

        // Test default code format
        match builder.get_code_format() {
            CodeFormat::SoliditySingleFile => {}
            _ => panic!("Expected SoliditySingleFile as default"),
        }
    }

    #[test]
    fn test_vyper_builder_minimal_configuration() {
        let client = TestUtils::create_test_client();
        let contracts = client.contracts();

        let builder = contracts.verify_vyper(TestUtils::contract_address());

        // Test default values
        assert!(builder.get_source_code().is_none());
        assert!(builder.get_contract_name().is_none());
        assert!(builder.get_compiler_version().is_none());
        assert!(!builder.get_optimization_settings().enabled);
        assert_eq!(builder.get_optimization_settings().runs, 0);
    }

    #[test]
    fn test_builder_method_chaining() {
        let client = TestUtils::create_test_client();
        let contracts = client.contracts();

        // Test that all methods return Self for chaining
        let _builder = contracts
            .verify_solidity(TestUtils::contract_address())
            .source_code("test")
            .contract_name("Test")
            .compiler_version("v0.8.24")
            .optimization(true, 200)
            .constructor_arguments("0x")
            .code_format(CodeFormat::SolidityStandardJsonInput)
            .library("Test", "0x1234567890123456789012345678901234567890")
            .license_type("MIT")
            .evm_version("default");

        // If we get here, chaining worked correctly
    }

    #[test]
    fn test_different_networks() {
        let networks = vec![
            Network::Mainnet,
            Network::Goerli,
            Network::Sepolia,
            Network::BinanceSmartChain,
            Network::Polygon,
        ];

        for network in networks {
            let client = TestUtils::create_test_client_for_network(network);
            let contracts = client.contracts();

            // Test that we can create builders for different networks
            let _solidity_builder = contracts.verify_solidity(TestUtils::contract_address());
            let _vyper_builder = contracts.verify_vyper(TestUtils::contract_address());
            let _proxy_builder = contracts.verify_proxy(TestUtils::contract_address());
        }
    }

    #[test]
    fn test_address_case_handling() {
        let client = TestUtils::create_test_client();
        let contracts = client.contracts();

        // Test with different address cases
        let addresses = vec![
            TestUtils::valid_address(),                   // lowercase
            TestUtils::valid_address_mixed_case(),        // mixed case
            "0X742D35CC6634C0532925A3B8D19389C4D5E1E4A6", // uppercase
        ];

        for address in addresses {
            let _builder = contracts.verify_solidity(address);
            let _vyper_builder = contracts.verify_vyper(address);
            let _proxy_builder = contracts.verify_proxy(address);
        }
    }
}

/// Test network-specific functionality
mod network_tests {
    use super::*;

    #[test]
    fn test_mainnet_contract_endpoints() {
        let client = TestUtils::create_test_client_for_network(Network::Mainnet);
        let contracts = client.contracts();

        // Test builder creation for mainnet
        let _builder = contracts.verify_solidity(TestUtils::contract_address());
        assert!(true); // If we get here, the builder was created successfully
    }

    #[test]
    fn test_testnet_contract_endpoints() {
        let testnets = vec![Network::Goerli, Network::Sepolia];

        for network in testnets {
            let client = TestUtils::create_test_client_for_network(network);
            let contracts = client.contracts();

            // Test builder creation for testnets
            let _builder = contracts.verify_solidity(TestUtils::contract_address());
            assert!(true); // If we get here, the builder was created successfully
        }
    }

    #[test]
    fn test_l2_contract_endpoints() {
        let l2_networks = vec![Network::Polygon, Network::Arbitrum, Network::Optimism];

        for network in l2_networks {
            let client = TestUtils::create_test_client_for_network(network);
            let contracts = client.contracts();

            // Test builder creation for L2 networks
            let _builder = contracts.verify_solidity(TestUtils::contract_address());
            assert!(true); // If we get here, the builder was created successfully
        }
    }

    #[test]
    fn test_bsc_contract_endpoints() {
        let client = TestUtils::create_test_client_for_network(Network::BinanceSmartChain);
        let contracts = client.contracts();

        // Test builder creation for BSC
        let _builder = contracts.verify_solidity(TestUtils::contract_address());
        assert!(true); // If we get here, the builder was created successfully
    }
}

/// Test builder pattern consistency
mod consistency_tests {
    use super::*;

    #[test]
    fn test_all_builders_have_address() {
        let client = TestUtils::create_test_client();
        let contracts = client.contracts();
        let test_address = TestUtils::contract_address();

        // All builders should accept the same address format
        let _solidity = contracts.verify_solidity(test_address);
        let _vyper = contracts.verify_vyper(test_address);
        let _proxy = contracts.verify_proxy(test_address);
    }

    #[test]
    fn test_getter_method_consistency() {
        let client = TestUtils::create_test_client();
        let contracts = client.contracts();

        // Test that all builders have consistent getter methods
        let solidity_builder = contracts
            .verify_solidity(TestUtils::contract_address())
            .source_code("test")
            .contract_name("Test")
            .compiler_version("v0.8.24");

        let vyper_builder = contracts
            .verify_vyper(TestUtils::contract_address())
            .source_code("test")
            .contract_name("Test")
            .compiler_version("v0.3.10");

        // Both should have consistent getter patterns
        assert!(solidity_builder.get_source_code().is_some());
        assert!(solidity_builder.get_contract_name().is_some());
        assert!(solidity_builder.get_compiler_version().is_some());

        assert!(vyper_builder.get_source_code().is_some());
        assert!(vyper_builder.get_contract_name().is_some());
        assert!(vyper_builder.get_compiler_version().is_some());
    }

    #[test]
    fn test_optimization_consistency() {
        let client = TestUtils::create_test_client();
        let contracts = client.contracts();

        // Both Solidity and Vyper builders should handle optimization the same way
        let solidity_builder = contracts
            .verify_solidity(TestUtils::contract_address())
            .optimization(true, 200);

        let vyper_builder = contracts
            .verify_vyper(TestUtils::contract_address())
            .optimization(false, 0);

        assert!(solidity_builder.get_optimization_settings().enabled);
        assert_eq!(solidity_builder.get_optimization_settings().runs, 200);

        assert!(!vyper_builder.get_optimization_settings().enabled);
        assert_eq!(vyper_builder.get_optimization_settings().runs, 0);
    }
}
