#![cfg(test)]
extern crate std;

use super::*;
use crate::error::ContractError;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{token, Address, Env, IntoVal, String};
use token::Client as TokenClient;
use token::StellarAssetClient;

const NFT_PRICE: u128 = 100;
const USER_STARTING_BALANCE: i128 = 1_000_000;

// --- Test Struct and Setup ---

struct MarketplaceTest {
    env: Env,
    admin: Address,
    creator: Address,
    buyer: Address,
    token_address: Address,
    contract_address: Address,
}

impl MarketplaceTest {
    /// Creates a new test environment with initialized accounts and contracts.
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let creator = Address::generate(&env);
        let buyer = Address::generate(&env);

        // Setup Token Contract
        let token = env.register_stellar_asset_contract_v2(admin.clone());
        let token_address = token.address();
        let token_admin_client = StellarAssetClient::new(&env, &token_address);
        token_admin_client.mint(&creator, &USER_STARTING_BALANCE);
        token_admin_client.mint(&buyer, &USER_STARTING_BALANCE);

        // Setup Marketplace Contract
        let contract_address = env.register(MarketplaceContract, ());
        let contract_client = MarketplaceContractClient::new(&env, &contract_address);
        contract_client.initialize(&admin);

        MarketplaceTest {
            env,
            admin,
            creator,
            buyer,
            token_address,
            contract_address,
        }
    }

    /// Returns a client for the marketplace contract.
    fn contract_client(&self) -> MarketplaceContractClient {
        MarketplaceContractClient::new(&self.env, &self.contract_address)
    }

    /// Returns a client for the token contract.
    fn token_client(&self) -> TokenClient {
        TokenClient::new(&self.env, &self.token_address)
    }

    /// Helper to create a default NFT for tests.
    fn create_nft(&self) -> u64 {
        let client = self.contract_client();
        client.create_nft(
            &self.creator,
            &String::from_str(&self.env, "Impact NFT"),
            &String::from_str(&self.env, "A great impact project"),
            &String::from_str(&self.env, "Carbon Credit"),
            &String::from_str(&self.env, "http://example.com/image.png"),
            &NFT_PRICE,
            &self.token_address,
        )
    }
}

// --- Tests ---

#[test]
fn test_initialize() {
    let test = MarketplaceTest::setup();
    let client = test.contract_client();

    // Try to initialize again, should fail.
    let result = client.try_initialize(&test.admin);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Ok(ContractError::AlreadyInitialized));
}

#[test]
fn test_create_nft() {
    let test = MarketplaceTest::setup();
    let client = test.contract_client();
    let nft_id = test.create_nft();

    assert_eq!(nft_id, 1);

    let nft = client.get_nft(&nft_id);
    assert_eq!(nft.id, 1);
    assert_eq!(nft.creator, test.creator);
    assert_eq!(nft.owner, test.creator);
    assert_eq!(nft.price, NFT_PRICE);
    assert_eq!(nft.status, NFTStatus::Available);
}

#[test]
fn test_create_nft_invalid_price() {
    let test = MarketplaceTest::setup();
    let client = test.contract_client();
    let result = client.try_create_nft(
        &test.creator,
        &String::from_str(&test.env, "Test"),
        &String::from_str(&test.env, "Test"),
        &String::from_str(&test.env, "Test"),
        &String::from_str(&test.env, "Test"),
        &0, // Invalid price
        &test.token_address,
    );
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Ok(ContractError::InvalidPrice));
}

#[test]
fn test_buy_nft() {
    let test = MarketplaceTest::setup();
    let contract_client = test.contract_client();
    let token_client = test.token_client();
    let nft_id = test.create_nft();

    let creator_balance_before = token_client.balance(&test.creator);
    let buyer_balance_before = token_client.balance(&test.buyer);

    contract_client.buy_nft(&test.buyer, &nft_id);

    let nft = contract_client.get_nft(&nft_id);
    assert_eq!(nft.owner, test.buyer);
    assert_eq!(nft.status, NFTStatus::Sold);

    // Check balances
    assert_eq!(
        token_client.balance(&test.buyer),
        buyer_balance_before - (NFT_PRICE as i128)
    );
    assert_eq!(
        token_client.balance(&test.creator),
        creator_balance_before + (NFT_PRICE as i128)
    );
}

#[test]
fn test_buy_nft_not_available() {
    let test = MarketplaceTest::setup();
    let client = test.contract_client();
    let nft_id = test.create_nft();
    client.toggle_nft_availability(&test.creator, &nft_id); // Make it unavailable

    let result = client.try_buy_nft(&test.buyer, &nft_id);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Ok(ContractError::NFTNotAvailable));
}

#[test]
fn test_buy_nft_already_owned() {
    let test = MarketplaceTest::setup();
    let client = test.contract_client();
    let nft_id = test.create_nft();

    // Creator tries to buy their own NFT
    let result = client.try_buy_nft(&test.creator, &nft_id);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Ok(ContractError::NFTAlreadyOwned));
}

#[test]
fn test_update_nft_price() {
    let test = MarketplaceTest::setup();
    let client = test.contract_client();
    let nft_id = test.create_nft();

    let new_price = 200;
    client.update_nft_price(&test.creator, &nft_id, &new_price);

    let nft = client.get_nft(&nft_id);
    assert_eq!(nft.price, new_price);
}

#[test]
fn test_update_nft_price_not_creator() {
    let test = MarketplaceTest::setup();
    let client = test.contract_client();
    let nft_id = test.create_nft();

    let new_price = 200;
    let result = client.try_update_nft_price(&test.buyer, &nft_id, &new_price);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), Ok(ContractError::CreatorOnly));
}

#[test]
fn test_toggle_nft_availability() {
    let test = MarketplaceTest::setup();
    let client = test.contract_client();
    let nft_id = test.create_nft();

    // Toggle to Unavailable
    client.toggle_nft_availability(&test.creator, &nft_id);
    let nft = client.get_nft(&nft_id);
    assert_eq!(nft.status, NFTStatus::Unavailable);

    // Toggle back to Available
    client.toggle_nft_availability(&test.creator, &nft_id);
    let nft = client.get_nft(&nft_id);
    assert_eq!(nft.status, NFTStatus::Available);
}

#[test]
fn test_get_functions() {
    let test = MarketplaceTest::setup();
    let client = test.contract_client();

    // Create some NFTs
    client.create_nft(&test.creator, &"NFT1".into_val(&test.env), &"Desc1".into_val(&test.env), &"CatA".into_val(&test.env), &"url1".into_val(&test.env), &100, &test.token_address);
    client.create_nft(&test.creator, &"NFT2".into_val(&test.env), &"Desc2".into_val(&test.env), &"CatB".into_val(&test.env), &"url2".into_val(&test.env), &200, &test.token_address);
    client.create_nft(&test.creator, &"NFT3".into_val(&test.env), &"Desc3".into_val(&test.env), &"CatA".into_val(&test.env), &"url3".into_val(&test.env), &300, &test.token_address);

    // Buy one
    client.buy_nft(&test.buyer, &2);

    // get_all_nfts
    assert_eq!(client.get_all_nfts().len(), 3);

    // get_nfts_by_category
    assert_eq!(client.get_nfts_by_category(&"CatA".into_val(&test.env)).len(), 2);
    assert_eq!(client.get_nfts_by_category(&"CatB".into_val(&test.env)).len(), 1);

    // get_nfts_by_creator
    assert_eq!(client.get_nfts_by_creator(&test.creator).len(), 3);

    // get_owned_nfts
    assert_eq!(client.get_owned_nfts(&test.creator).len(), 2);
    assert_eq!(client.get_owned_nfts(&test.buyer).len(), 1);

    // get_marketplace_stats
    let stats = client.get_marketplace_stats();
    assert_eq!(stats.get(String::from_str(&test.env, "total_nfts")), Some(3));
    assert_eq!(stats.get(String::from_str(&test.env, "available_nfts")), Some(2));
    assert_eq!(stats.get(String::from_str(&test.env, "sold_nfts")), Some(1));
}
