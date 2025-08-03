mod common;

use common::{TestUtils, TestConstants};
use keion_etherscan::{EtherscanClient, Network, Sort, TransactionType};

/// Test all account endpoint builders (non-async tests)
mod builder_tests {
    use super::*;

    #[test]
    fn test_transaction_query_builder() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        let query = accounts
            .transactions(TestUtils::valid_address())
            .page(1)
            .offset(TestConstants::STANDARD_PAGE_SIZE)
            .start_block(TestConstants::OLD_BLOCK)
            .end_block(TestConstants::RECENT_BLOCK)
            .sort(Sort::Ascending);

        assert_eq!(query.get_pagination().page, Some(1));
        assert_eq!(query.get_pagination().offset, Some(TestConstants::STANDARD_PAGE_SIZE));
        assert_eq!(query.get_pagination().start_block, Some(TestConstants::OLD_BLOCK));
        assert_eq!(query.get_pagination().end_block, Some(TestConstants::RECENT_BLOCK));
        assert_eq!(query.get_pagination().sort, Some(Sort::Ascending));
        assert_eq!(query.get_tx_type(), TransactionType::Normal);
    }

    #[test]
    fn test_token_transfer_query_builder() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        let query = accounts
            .token_transfers(TestUtils::valid_address())
            .contract_address(TestUtils::contract_address())
            .page(2)
            .offset(50);

        assert!(query.get_contract_address().is_some());
        assert_eq!(query.get_contract_address().as_ref().unwrap(), TestUtils::contract_address());
        assert_eq!(query.get_pagination().page, Some(2));
        assert_eq!(query.get_pagination().offset, Some(50));
        assert_eq!(query.get_tx_type(), TransactionType::Token);
    }

    #[test]
    fn test_internal_transactions_by_address_builder() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        let query = accounts
            .internal_transactions()
            .by_address(TestUtils::valid_address())
            .page(1)
            .offset(50)
            .start_block(TestConstants::OLD_BLOCK)
            .end_block(TestConstants::RECENT_BLOCK)
            .sort(Sort::Descending);

        assert_eq!(query.get_address(), TestUtils::valid_address());
        assert_eq!(query.get_pagination().page, Some(1));
        assert_eq!(query.get_pagination().offset, Some(50));
        assert_eq!(query.get_pagination().start_block, Some(TestConstants::OLD_BLOCK));
        assert_eq!(query.get_pagination().end_block, Some(TestConstants::RECENT_BLOCK));
        assert_eq!(query.get_pagination().sort, Some(Sort::Descending));
    }

    #[test]
    fn test_internal_transactions_by_hash_builder() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        let tx_hash = TestUtils::valid_tx_hash();
        let query = accounts
            .internal_transactions()
            .by_hash(tx_hash);

        assert_eq!(query.get_tx_hash(), tx_hash);
    }

    #[test]
    fn test_internal_transactions_by_block_range_builder() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        let start_block = TestConstants::OLD_BLOCK;
        let end_block = TestConstants::RECENT_BLOCK;
        let query = accounts
            .internal_transactions()
            .by_block_range(start_block, end_block)
            .page(2)
            .offset(TestConstants::STANDARD_PAGE_SIZE)
            .sort(Sort::Ascending);

        assert_eq!(query.get_start_block(), start_block);
        assert_eq!(query.get_end_block(), end_block);
        assert_eq!(query.get_pagination().page, Some(2));
        assert_eq!(query.get_pagination().offset, Some(TestConstants::STANDARD_PAGE_SIZE));
        assert_eq!(query.get_pagination().sort, Some(Sort::Ascending));
    }

    #[test]
    fn test_validated_blocks_builder() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        let validator_address = TestUtils::validator_address();
        let query = accounts
            .blocks_validated(validator_address)
            .page(1)
            .offset(25);

        assert_eq!(query.get_address(), validator_address);
        assert_eq!(query.get_pagination().page, Some(1));
        assert_eq!(query.get_pagination().offset, Some(25));
    }

    #[test]
    fn test_beacon_withdrawals_builder() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        let validator_address = TestUtils::validator_address();
        let query = accounts
            .beacon_withdrawals(validator_address)
            .start_block(TestConstants::MAINNET_BLOCK)
            .end_block(TestConstants::RECENT_BLOCK)
            .page(1)
            .offset(TestConstants::STANDARD_PAGE_SIZE)
            .sort(Sort::Descending);

        assert_eq!(query.get_address(), validator_address);
        assert_eq!(query.get_start_block(), Some(TestConstants::MAINNET_BLOCK));
        assert_eq!(query.get_end_block(), Some(TestConstants::RECENT_BLOCK));
        assert_eq!(query.get_pagination().page, Some(1));
        assert_eq!(query.get_pagination().offset, Some(TestConstants::STANDARD_PAGE_SIZE));
        assert_eq!(query.get_pagination().sort, Some(Sort::Descending));
    }

    #[test]
    fn test_beacon_withdrawals_block_range_convenience() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        let start_block = TestConstants::MAINNET_BLOCK;
        let end_block = TestConstants::RECENT_BLOCK;
        let query = accounts
            .beacon_withdrawals(TestUtils::valid_address())
            .block_range(start_block, end_block);

        assert_eq!(query.get_start_block(), Some(start_block));
        assert_eq!(query.get_end_block(), Some(end_block));
    }

    #[test]
    fn test_historical_balance_builder() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        let address = TestUtils::valid_address();
        let block_number = TestConstants::MAINNET_BLOCK;
        let query = accounts
            .historical_balance(address)
            .at_block(block_number);

        assert_eq!(query.get_address(), address);
        assert_eq!(query.get_block_number(), Some(block_number));
    }

    #[test]
    fn test_historical_balance_defaults_to_latest() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        let query = accounts
            .historical_balance(TestUtils::valid_address());

        // Without at_block, should default to None (latest)
        assert_eq!(query.get_block_number(), None);
    }
}

/// Test edge cases and boundary conditions
mod edge_case_tests {
    use super::*;

    #[test]
    fn test_address_case_handling() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        // Test with mixed case address
        let mixed_case = TestUtils::valid_address_mixed_case();
        let query = accounts.transactions(mixed_case);
        
        // Address should be stored as provided (normalization happens in execute)
        assert_eq!(query.get_address(), mixed_case);
    }

    #[test]
    fn test_pagination_edge_cases() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();
        
        // Test very large pagination values
        let query = accounts
            .transactions(TestUtils::valid_address())
            .page(u32::MAX)
            .offset(TestConstants::MAX_PAGE_SIZE);
        
        assert_eq!(query.get_pagination().page, Some(u32::MAX));
        assert_eq!(query.get_pagination().offset, Some(TestConstants::MAX_PAGE_SIZE));
    }

    #[test]
    fn test_zero_values() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();
        
        // Test zero/minimal values
        let query = accounts
            .beacon_withdrawals(TestUtils::valid_address())
            .start_block(0)
            .end_block(0)
            .page(0)
            .offset(0);
        
        assert_eq!(query.get_start_block(), Some(0));
        assert_eq!(query.get_end_block(), Some(0));
        assert_eq!(query.get_pagination().page, Some(0));
        assert_eq!(query.get_pagination().offset, Some(0));
    }

    #[test]
    fn test_method_chaining_flexibility() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        // Test method chaining in different orders
        let query1 = accounts
            .internal_transactions()
            .by_address(TestUtils::valid_address())
            .block_range(TestConstants::OLD_BLOCK, TestConstants::RECENT_BLOCK)
            .page(1);

        let query2 = accounts
            .internal_transactions()
            .by_address(TestUtils::valid_address())
            .page(1)
            .block_range(TestConstants::OLD_BLOCK, TestConstants::RECENT_BLOCK);

        assert_eq!(query1.get_pagination().start_block, Some(TestConstants::OLD_BLOCK));
        assert_eq!(query1.get_pagination().end_block, Some(TestConstants::RECENT_BLOCK));
        assert_eq!(query1.get_pagination().page, Some(1));

        assert_eq!(query2.get_pagination().start_block, Some(TestConstants::OLD_BLOCK));
        assert_eq!(query2.get_pagination().end_block, Some(TestConstants::RECENT_BLOCK));
        assert_eq!(query2.get_pagination().page, Some(1));
    }

    #[test]
    fn test_all_transaction_types() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();
        let address = TestUtils::valid_address();

        // Normal transactions
        let normal_query = accounts.transactions(address);
        assert_eq!(normal_query.get_tx_type(), TransactionType::Normal);

        // Token transfers
        let token_query = accounts.token_transfers(address);
        assert_eq!(token_query.get_tx_type(), TransactionType::Token);

        // NFT transfers
        let nft_query = accounts.nft_transfers(address);
        assert_eq!(nft_query.get_tx_type(), TransactionType::TokenNft);

        // ERC-1155 transfers
        let erc1155_query = accounts.erc1155_transfers(address);
        assert_eq!(erc1155_query.get_tx_type(), TransactionType::Token1155);
    }

    #[test]
    fn test_parameter_validation_boundary_cases() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();
        
        // Test that builders accept boundary cases
        // (Note: These would fail at runtime during execute(), not at builder creation)
        
        // Invalid transaction hash length (builder should still work)
        let _invalid_hash_query = accounts
            .internal_transactions()
            .by_hash(TestUtils::invalid_tx_hash_too_short());
        
        // Block range where end < start (builder should still work)
        let _block_range_query = accounts
            .internal_transactions()
            .by_block_range(TestConstants::RECENT_BLOCK, TestConstants::OLD_BLOCK);
        
        // These compile but would fail validation in execute()
    }

    #[test]
    fn test_builder_pattern_consistency() {
        let client = TestUtils::create_test_client();
        let accounts = client.accounts();

        // Test that all builders follow the same pattern and compile
        let _balance_query = accounts.historical_balance(TestUtils::valid_address());
        let _blocks_query = accounts.blocks_validated(TestUtils::validator_address());
        let _withdrawals_query = accounts.beacon_withdrawals(TestUtils::validator_address());
        let _internal_tx_query = accounts.internal_transactions();

        // All should compile and follow builder pattern
    }
}

/// Test multi-network support
mod network_tests {
    use super::*;

    #[test]
    fn test_different_networks() {
        // Test that builders work with different networks
        let networks = vec![
            Network::Mainnet,
            Network::Goerli,
            Network::Sepolia,
            Network::Polygon,
            Network::BinanceSmartChain,
        ];

        for network in networks {
            let client = TestUtils::create_test_client_for_network(network);
            let accounts = client.accounts();

            // All endpoints should work regardless of network
            let _balance_query = accounts.historical_balance(TestUtils::valid_address());
            let _tx_query = accounts.transactions(TestUtils::valid_address());
            let _internal_query = accounts.internal_transactions();
            
            assert_eq!(client.network(), network);
        }
    }

    #[test]
    fn test_network_specific_features() {
        // Beacon withdrawals are only available on mainnet post-merge
        let mainnet_client = TestUtils::create_test_client_for_network(Network::Mainnet);
        let accounts = mainnet_client.accounts();
        
        let _withdrawals_query = accounts.beacon_withdrawals(TestUtils::validator_address());
        
        // Should compile for all networks, but API would return errors for non-mainnet
        let goerli_client = TestUtils::create_test_client_for_network(Network::Goerli);
        let goerli_accounts = goerli_client.accounts();
        
        let _goerli_withdrawals_query = goerli_accounts.beacon_withdrawals(TestUtils::validator_address());
    }
}