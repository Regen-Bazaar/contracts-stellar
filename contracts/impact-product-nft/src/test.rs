#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_init_and_mint() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ImpactProductNFT);
    let client = ImpactProductNFTClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let base_uri = String::from_str(&env, "https://example.com/nft/");

    client.init(&admin, &base_uri);

    // Mint an NFT
    let impact_data = ImpactData {
        category: String::from_str(&env, "Education"),
        impact_value: 100,
        location: String::from_str(&env, "Nairobi"),
        start_date: 1_700_000_000,
        end_date: 1_700_086_400,
        beneficiaries: String::from_str(&env, "Children"),
        verified: false,
        metadata_uri: String::from_str(&env, "meta/1.json"),
    };
    let price = 1_000_000;
    let royalty_bps = 500;
    let token_id = client.mint(&user1, &impact_data, &price, &admin, &royalty_bps);

    // Query NFT info
    let info = client.get_token_info(&token_id).unwrap();
    assert_eq!(info.owner, user1);
    assert_eq!(info.impact_data.category, String::from_str(&env, "Education"));
    assert_eq!(info.price, price);
    assert_eq!(info.royalty.royalty_bps, royalty_bps);

    // Query by creator
    let tokens = client.get_tokens_by_creator(&user1);
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens.get(0), Some(token_id));

    // Query by category
    let cat_tokens = client.get_tokens_by_category(&String::from_str(&env, "Education"));
    assert_eq!(cat_tokens.len(), 1);
    assert_eq!(cat_tokens.get(0), Some(token_id));

    // Query all tokens
    let all_tokens = client.get_all_tokens();
    assert_eq!(all_tokens.len(), 1);
    assert_eq!(all_tokens.get(0), Some(token_id));
} 