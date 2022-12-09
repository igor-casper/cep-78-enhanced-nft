use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR, DEFAULT_PROPOSER_ADDR,
    DEFAULT_RUN_GENESIS_REQUEST,
};
use casper_types::{account::AccountHash, runtime_args, ContractHash, Key, RuntimeArgs};

use crate::utility::{
    constants::{
        ARG_IS_HASH_IDENTIFIER_MODE, ARG_NFT_CONTRACT_HASH, ARG_SOURCE_KEY, ARG_TARGET_KEY,
        ARG_TOKEN_ID, ARG_TOKEN_META_DATA, ARG_TOKEN_OWNER, CONTRACT_NAME, MINT_SESSION_WASM,
        NFT_CONTRACT_WASM, NFT_TEST_COLLECTION, NFT_TEST_SYMBOL, TRANSFER_SESSION_WASM,
    },
    installer_request_builder::{
        InstallerRequestBuilder, NFTIdentifierMode, NFTMetadataKind, OwnershipMode,
    },
    support,
};

#[test]
fn mint_cost_should_remain_stable() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let install_request = InstallerRequestBuilder::new(*DEFAULT_ACCOUNT_ADDR, NFT_CONTRACT_WASM)
        .with_collection_name(NFT_TEST_COLLECTION.to_string())
        .with_collection_symbol(NFT_TEST_SYMBOL.to_string())
        .with_total_token_supply(100u64)
        .with_ownership_mode(OwnershipMode::Minter)
        .with_identifier_mode(NFTIdentifierMode::Ordinal)
        .with_nft_metadata_kind(NFTMetadataKind::Raw)
        .build();

    builder.exec(install_request).expect_success().commit();

    let nft_contract_hash = support::get_nft_contract_hash(&builder);
    let nft_contract_key: Key = nft_contract_hash.into();

    let first_mint_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        MINT_SESSION_WASM,
        runtime_args! {
            ARG_NFT_CONTRACT_HASH => nft_contract_key,
            ARG_TOKEN_OWNER => Key::Account(*DEFAULT_ACCOUNT_ADDR),
            ARG_TOKEN_META_DATA => "",
        },
    )
    .build();

    builder.exec(first_mint_request).expect_success().commit();

    let second_mint_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        MINT_SESSION_WASM,
        runtime_args! {
            ARG_NFT_CONTRACT_HASH => nft_contract_key,
            ARG_TOKEN_OWNER => Key::Account(*DEFAULT_ACCOUNT_ADDR),
            ARG_TOKEN_META_DATA => "",
        },
    )
    .build();

    builder.exec(second_mint_request).expect_success().commit();

    // We check only the second and third gas costs as the first mint cost
    // has the additional gas of allocating a whole new page. Thus we ensure
    // that costs once a page has been allocated remain stable.
    let second_mint_gas_costs = builder.last_exec_gas_cost();

    let third_mint_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        MINT_SESSION_WASM,
        runtime_args! {
            ARG_NFT_CONTRACT_HASH => nft_contract_key,
            ARG_TOKEN_OWNER => Key::Account(*DEFAULT_ACCOUNT_ADDR),
            ARG_TOKEN_META_DATA => "",
        },
    )
    .build();

    builder.exec(third_mint_request).expect_success().commit();

    let third_mint_gas_costs = builder.last_exec_gas_cost();

    assert_eq!(second_mint_gas_costs, third_mint_gas_costs);
}

#[test]
fn transfer_costs_should_remain_stable() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let install_request = InstallerRequestBuilder::new(*DEFAULT_ACCOUNT_ADDR, NFT_CONTRACT_WASM)
        .with_collection_name(NFT_TEST_COLLECTION.to_string())
        .with_collection_symbol(NFT_TEST_SYMBOL.to_string())
        .with_total_token_supply(100u64)
        .with_ownership_mode(OwnershipMode::Transferable)
        .with_identifier_mode(NFTIdentifierMode::Ordinal)
        .with_nft_metadata_kind(NFTMetadataKind::Raw)
        .build();

    builder.exec(install_request).expect_success().commit();

    let nft_contract_hash = support::get_nft_contract_hash(&builder);
    let nft_contract_key: Key = nft_contract_hash.into();

    for _ in 0..3 {
        let mint_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            MINT_SESSION_WASM,
            runtime_args! {
                ARG_NFT_CONTRACT_HASH => nft_contract_key,
                ARG_TOKEN_OWNER => Key::Account(*DEFAULT_ACCOUNT_ADDR),
                ARG_TOKEN_META_DATA => "",
            },
        )
        .build();

        builder.exec(mint_request).expect_success().commit();
    }

    let first_transfer_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        TRANSFER_SESSION_WASM,
        runtime_args! {
            ARG_NFT_CONTRACT_HASH => nft_contract_key,
            ARG_IS_HASH_IDENTIFIER_MODE => false,
            ARG_SOURCE_KEY => Key::Account(*DEFAULT_ACCOUNT_ADDR),
            ARG_TARGET_KEY => Key::Account(AccountHash::new([9u8; 32])),
            ARG_TOKEN_ID => 0u64,
        },
    )
    .build();

    builder
        .exec(first_transfer_request)
        .expect_success()
        .commit();

    let second_transfer_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        TRANSFER_SESSION_WASM,
        runtime_args! {
            ARG_NFT_CONTRACT_HASH => nft_contract_key,
            ARG_IS_HASH_IDENTIFIER_MODE => false,
            ARG_SOURCE_KEY => Key::Account(*DEFAULT_ACCOUNT_ADDR),
            ARG_TARGET_KEY => Key::Account(AccountHash::new([9u8; 32])),
            ARG_TOKEN_ID => 1u64,
        },
    )
    .build();

    builder
        .exec(second_transfer_request)
        .expect_success()
        .commit();

    // We check only the second and third gas costs as the first transfer cost
    // has the additional gas of allocating a whole new page. Thus we ensure
    // that costs once a page has been allocated remain stable.
    let second_transfer_gas_cost = builder.last_exec_gas_cost();

    let third_transfer_request = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        TRANSFER_SESSION_WASM,
        runtime_args! {
            ARG_NFT_CONTRACT_HASH => nft_contract_key,
            ARG_IS_HASH_IDENTIFIER_MODE => false,
            ARG_SOURCE_KEY => Key::Account(*DEFAULT_ACCOUNT_ADDR),
            ARG_TARGET_KEY => Key::Account(AccountHash::new([9u8; 32])),
            ARG_TOKEN_ID => 2u64,
        },
    )
    .build();

    builder
        .exec(third_transfer_request)
        .expect_success()
        .commit();

    let third_transfer_gas_cost = builder.last_exec_gas_cost();

    assert_eq!(second_transfer_gas_cost, third_transfer_gas_cost);
}

#[test]
fn should_have_same_costs_for_mint_with_different_total_supply() {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();

    let install_request_1 = InstallerRequestBuilder::new(*DEFAULT_ACCOUNT_ADDR, NFT_CONTRACT_WASM)
        .with_collection_name(NFT_TEST_COLLECTION.to_string())
        .with_collection_symbol(NFT_TEST_SYMBOL.to_string())
        .with_total_token_supply(10000u64)
        .with_ownership_mode(OwnershipMode::Transferable)
        .with_identifier_mode(NFTIdentifierMode::Ordinal)
        .with_nft_metadata_kind(NFTMetadataKind::Raw)
        .build();

    builder.exec(install_request_1).expect_success().commit();

    let nft_contract_hash_1000 = support::get_nft_contract_hash(&builder);
    let nft_contract_key_1000: Key = nft_contract_hash_1000.into();

    let install_request_2 = InstallerRequestBuilder::new(*DEFAULT_PROPOSER_ADDR, NFT_CONTRACT_WASM)
        .with_collection_name(NFT_TEST_COLLECTION.to_string())
        .with_collection_symbol(NFT_TEST_SYMBOL.to_string())
        .with_total_token_supply(100u64)
        .with_ownership_mode(OwnershipMode::Transferable)
        .with_identifier_mode(NFTIdentifierMode::Ordinal)
        .with_nft_metadata_kind(NFTMetadataKind::Raw)
        .build();

    builder.exec(install_request_2).expect_success().commit();

    // Mint into the contract with a total supply of a 10000.
    let mint_request_1 = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        MINT_SESSION_WASM,
        runtime_args! {
            ARG_NFT_CONTRACT_HASH => nft_contract_key_1000,
            ARG_TOKEN_OWNER => Key::Account(*DEFAULT_ACCOUNT_ADDR),
            ARG_TOKEN_META_DATA => "",
        },
    )
    .build();

    builder.exec(mint_request_1).expect_success().commit();

    let mint_execution_cost = builder.last_exec_gas_cost();

    let nft_hash_addr_100 = builder
        .get_expected_account(*DEFAULT_PROPOSER_ADDR)
        .named_keys()
        .get(CONTRACT_NAME)
        .expect("must have this entry in named keys")
        .into_hash()
        .expect("must get hash_addr");

    let nft_contract_100: Key = ContractHash::new(nft_hash_addr_100).into();

    let mint_request_2 = ExecuteRequestBuilder::standard(
        *DEFAULT_PROPOSER_ADDR,
        MINT_SESSION_WASM,
        runtime_args! {
            ARG_NFT_CONTRACT_HASH => nft_contract_100,
            ARG_TOKEN_OWNER => Key::Account(*DEFAULT_PROPOSER_ADDR),
            ARG_TOKEN_META_DATA => "",
        },
    )
    .build();

    builder.exec(mint_request_2).expect_success().commit();

    let other_mint_execution_cost = builder.last_exec_gas_cost();

    assert_eq!(mint_execution_cost, other_mint_execution_cost);
}
