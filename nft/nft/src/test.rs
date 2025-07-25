#[cfg(test)]
use crate::{NFT, NFTClient};
#[cfg(test)]
use soroban_sdk::{Env, String, Address};
#[cfg(test)]
use soroban_sdk::testutils::{Address as _};

#[test]
fn test_name() {
    let env: Env = Env::default();
    let contract_id: Address = env.register(NFT, ());
    let client: NFTClient<'_> = NFTClient::new(&env, &contract_id);
    assert_eq!(client.name(), String::from_str(&env, "NFT"));
}

#[test]
fn test_symbol() {
    let env: Env = Env::default();
    let contract_id: Address = env.register(NFT, ());
    let client: NFTClient<'_> = NFTClient::new(&env, &contract_id);
    assert_eq!(client.symbol(), String::from_str(&env, "NFT"));
}

#[test]
fn test_token_uri() {
    let env: Env = Env::default();
    let contract_id: Address = env.register(NFT, ());
    let client: NFTClient<'_> = NFTClient::new(&env, &contract_id);
    assert_eq!(
        client.token_uri(),
        String::from_str(
            &env,
            "https://ipfs.io/ipfs/QmegWR31kiQcD9S2katTXKxracbAgLs2QLBRGruFW3NhXC"
        )
    );
}

#[test]
fn test_token_image() {
    let env: Env = Env::default();
    let contract_id: Address = env.register(NFT, ());
    let client: NFTClient<'_> = NFTClient::new(&env, &contract_id);
    assert_eq!(
        client.token_image(),
        String::from_str(
            &env,
            "https://ipfs.io/ipfs/QmeRHSYkR4aGRLQXaLmZiccwHw7cvctrB211DzxzuRiqW6"
        )
    );
}

#[test]
fn test_mint() {
    let env: Env = Env::default();
    let contract_id: Address = env.register(NFT, ());
    let client: NFTClient<'_> = NFTClient::new(&env, &contract_id);
    let to: Address = Address::generate(&env);
    client.mint(&to);
    let token_id: i128 = 1; // Since it's the first token minted
    assert_eq!(client.owner_of(&token_id), to);
}

#[test]
fn test_owner_of() {
    let env: Env = Env::default();
    let contract_id: Address = env.register(NFT, ());
    let client: NFTClient<'_> = NFTClient::new(&env, &contract_id);
    let owner: Address = Address::generate(&env);
    client.mint(&owner);
    let token_id: i128 = 1;
    assert_eq!(client.owner_of(&token_id), owner);
}

#[test]
fn test_transfer() {
    let env: Env = Env::default();
    let contract_id: Address = env.register(NFT, ());
    let client: NFTClient<'_> = NFTClient::new(&env, &contract_id);
    let owner: Address = Address::generate(&env);
    let to: Address = Address::generate(&env);
    env.mock_all_auths();
    client.mint(&owner);
    let token_id: i128 = 1;
    client.transfer(&owner, &to, &token_id);
    assert_eq!(client.owner_of(&token_id), to);
}


#[test]
fn test_approve_and_is_approved() {
    let env: Env = Env::default();
    let contract_id: Address = env.register(NFT, ());
    let client: NFTClient<'_> = NFTClient::new(&env, &contract_id);
    let owner: Address = Address::generate(&env);
    let operator: Address = Address::generate(&env);
    env.mock_all_auths();
    client.mint(&owner);
    let token_id: i128 = 1;
    client.approve(&owner, &operator, &token_id);
    assert!(client.is_approved(&operator, &token_id));
}

#[test]
fn test_is_approved_false() {
    let env: Env = Env::default();
    let contract_id: Address = env.register(NFT, ());
    let client: NFTClient<'_> = NFTClient::new(&env, &contract_id);
    let owner: Address = Address::generate(&env);
    let operator: Address = Address::generate(&env);
    client.mint(&owner);
    let token_id: i128 = 1;
    assert!(!client.is_approved(&operator, &token_id));
}

#[test]
fn test_transfer_from() {
    let env: Env = Env::default();
    let contract_id: Address = env.register(NFT, ());
    let client: NFTClient<'_> = NFTClient::new(&env, &contract_id);
    let owner: Address = Address::generate(&env);
    let operator: Address = Address::generate(&env);
    let to: Address = Address::generate(&env);
    env.mock_all_auths();
    client.mint(&owner);
    let token_id: i128 = 1;
    client.approve(&owner, &operator, &token_id);
    client.transfer_from(&operator, &owner, &to, &token_id);
    assert_eq!(client.owner_of(&token_id), to);
}

#[test]
#[should_panic(expected = "Not the token owner")]
fn test_transfer_not_owner() {
    let env: Env = Env::default();
    let contract_id: Address = env.register(NFT, ());
    let client: NFTClient<'_> = NFTClient::new(&env, &contract_id);
    let owner: Address = Address::generate(&env);
    let not_owner: Address = Address::generate(&env);
    let to: Address = Address::generate(&env);
    env.mock_all_auths();
    client.mint(&owner);
    let token_id: i128 = 1;
    client.transfer(&not_owner, &to, &token_id);
}

#[test]
#[should_panic(expected = "Spender is not approved for this token")]
fn test_transfer_from_not_approved() {
    let env: Env = Env::default();
    let contract_id: Address = env.register(NFT, ());
    let client: NFTClient<'_> = NFTClient::new(&env, &contract_id);
    let owner: Address = Address::generate(&env);
    let operator: Address = Address::generate(&env);
    let to: Address = Address::generate(&env);
    env.mock_all_auths();
    client.mint(&owner);
    let token_id: i128 = 1;
    client.transfer_from(&operator, &owner, &to, &token_id);
}

#[test]
#[should_panic(expected = "From not owner")]
fn test_transfer_from_wrong_owner() {
    let env: Env = Env::default();
    let contract_id: Address = env.register(NFT, ());
    let client: NFTClient<'_> = NFTClient::new(&env, &contract_id);
    let owner: Address = Address::generate(&env);
    let wrong_owner: Address = Address::generate(&env);
    let operator: Address = Address::generate(&env);
    let to: Address = Address::generate(&env);
    env.mock_all_auths();
    client.mint(&owner);
    let token_id: i128 = 1;
    client.approve(&owner, &operator, &token_id);
    client.transfer_from(&operator, &wrong_owner, &to, &token_id);
}