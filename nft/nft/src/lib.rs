#![no_std]
mod contract;

use soroban_sdk::{contract, contractimpl, contracttype, log, symbol_short, Address, Bytes, Env, Map, String, Vec};

#[contract]
pub struct ImpactProductNFT;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImpactData {
    pub category: String,
    pub impact_value: u128,
    pub location: String,
    pub start_date: u128,
    pub end_date: u128,
    pub beneficiaries: String,
    pub verfied: bool,
    pub metadata_uri: String
}

#[contracttype]
pub enum DataKey {
    Owner(u128),
    TokenCount,
    Approvals(u128),
    BASEURI,
    ADMIN,
    MINTER,
    VERIFIER,
    PAUSER,
    TokenURIs,
    ImpactData,
    TokenPrices,
    CreatorTokens,
    CategoryTokens
}

#[contractimpl]
impl ImpactProductNFT {
    const SUPPLY: u128 = 1000;
    const NAME: &'static str = "Regen Bazaar Impact Product";
    const SYMBOL: &'static str = "RIP";

    pub fn __constructor(env: Env, admin: Address, base_token_uri: String) {
        env.storage().instance().set(&DataKey::ADMIN, &admin);
        env.storage().instance().set(&DataKey::MINTER, &admin);
        env.storage().instance().set(&DataKey::VERIFIER, &admin);
        env.storage().instance().set(&DataKey::PAUSER, &admin);

        env.storage().instance().set(&DataKey::BASEURI, &base_token_uri);

        let token_uris: Map<u128, String> = Map::new(&env);
        env.storage().persistent().set(&DataKey::TokenURIs, &token_uris);

        let impact_data: Map<u128, ImpactData> = Map::new(&env);
        env.storage().persistent().set(&DataKey::ImpactData, &impact_data);

        let token_prices: Map<u128, u128> = Map::new(&env);
        env.storage().persistent().set(&DataKey::TokenPrices, &token_prices);

        let creator_tokens: Map<Address, Vec<u128>> = Map::new(&env);
        env.storage().persistent().set(&DataKey::CreatorTokens, &creator_tokens);

        let category_tokens: Map<String, Vec<u128>> = Map::new(&env);
        env.storage().persistent().set(&DataKey::CategoryTokens, &category_tokens);
    }

    pub fn token_count(env: Env) -> u128 {
        let data: u128 = env.storage().persistent().get(&DataKey::TokenCount).unwrap_or_else(|| {0});
        data
    }

    pub fn owner_of(env: Env, token_id: u128) -> Address {
        env.storage().persistent().get(&DataKey::Owner(token_id)).unwrap_or_else(|| {
            Address::from_string_bytes(&Bytes::from_slice(&env, &[0; 32]))
        })
    }

    pub fn name(env: Env) -> String {
        String::from_str(&env, Self::NAME)
    }

    pub fn symbol(env: Env) -> String {
        String::from_str(&env, Self::SYMBOL)
    }

    pub fn token_uri(env: Env) -> String {
        let data: String = env.storage().instance().get(&DataKey::BASEURI).expect("should contain uri");
        data
    }

    pub fn create_impact_product(env: Env, to: Address, impact_data: ImpactData, price: u128) {
        if !(String::len(&impact_data.category) > 0) {
            panic!("Category cannot be empty")
        }
        if !(impact_data.impact_value > 0) {
            panic!("Impact value must be positive")
        }
        if !(price > 0) {
            panic!("Price must be positive")
        }

        Self::mint(env.clone(), to.clone());

        let mut current_id: u128 = env.storage().persistent().get(&DataKey::TokenCount).expect("Current ID not found");
        current_id += 1;

        if String::len(&impact_data.metadata_uri) > 0 {
            Self::_set_token_uri(env.clone(), current_id, impact_data.metadata_uri.clone());
        }

        let mut impact_data_all: Map<u128, ImpactData> = env.storage().persistent().get(&DataKey::ImpactData).expect("Impact Data Map not found");
        impact_data_all.set(current_id, impact_data.clone());
        env.storage().persistent().set(&DataKey::ImpactData, &impact_data_all);

        let mut token_prices_all: Map<u128, u128> = env.storage().persistent().get(&DataKey::TokenPrices).expect("Token Prices Map not found");
        token_prices_all.set(current_id, price);
        env.storage().persistent().set(&DataKey::TokenPrices, &token_prices_all);

        let mut creator_tokens_all: Map<Address, Vec<u128>> = env.storage().persistent().get(&DataKey::CreatorTokens).expect("Creator Tokens Map not found");
        let creators_tokens_user: bool = creator_tokens_all.contains_key(to.clone());

        if creators_tokens_user {
            let creators_tokens_user_array: Option<Vec<u128>> = creator_tokens_all.get(to.clone());
            let mut user_tokens_array: Vec<u128> = creators_tokens_user_array.unwrap();
            user_tokens_array.push_back(current_id);
            creator_tokens_all.set(to, user_tokens_array);
            env.storage().instance().set(&DataKey::CreatorTokens, &creator_tokens_all);
        } else {
            let mut user_tokens_array: Vec<u128> = Vec::new(&env);
            user_tokens_array.push_back(current_id);
            creator_tokens_all.set(to, user_tokens_array);
            env.storage().instance().set(&DataKey::CreatorTokens, &creator_tokens_all);
        }

        let mut category_tokens_all: Map<String, Vec<u128>> = env.storage().persistent().get(&DataKey::CategoryTokens).expect("Category Tokens Map not found");
        let category_tokens: bool = category_tokens_all.contains_key(impact_data.category.clone());
        if category_tokens {
            let category_tokens_category: Option<Vec<u128>> = category_tokens_all.get(impact_data.category.clone());
            let mut category_tokens_category_array: Vec<u128> = category_tokens_category.unwrap();
            category_tokens_category_array.push_back(current_id);
            category_tokens_all.set(impact_data.category, category_tokens_category_array);
            env.storage().instance().set(&DataKey::CreatorTokens, &category_tokens_all);
        } else {
            let mut category_tokens_array: Vec<u128> = Vec::new(&env);
            category_tokens_array.push_back(current_id);
            category_tokens_all.set(impact_data.category, category_tokens_array);
            env.storage().instance().set(&DataKey::CreatorTokens, &category_tokens_all);
        }
    }

    fn _set_token_uri(env: Env, token_id: u128, token_uri: String) {
        let mut token_uris: Map<u128, String> = env.storage().persistent().get(&DataKey::TokenURIs).expect("No Token URIS Map");
        
        token_uris.set(token_id, token_uri);
        
        env.storage().instance().set(&DataKey::TokenURIs, &token_uris);
    }

    pub fn is_approved(env: Env, operator: Address, token_id: u128) -> bool {
        let key: DataKey = DataKey::Approvals(token_id);
        let approvals: Vec<Address> = env.storage().persistent().get::<DataKey, Vec<Address>>(&key).unwrap_or_else(|| Vec::new(&env));
        approvals.contains(&operator)
    }

    pub fn transfer(env: Env, owner: Address, to: Address, token_id: u128) {
        owner.require_auth();
        let actual_owner: Address = Self::owner_of(env.clone(), token_id);
        if owner == actual_owner {
            env.storage().persistent().set(&DataKey::Owner(token_id), &to);
            env.storage().persistent().remove(&DataKey::Approvals(token_id));
            env.events().publish((symbol_short!("Transfer"),), (owner, to, token_id));
        } else {
            panic!("Not the token owner");
        }
    }

    fn mint(env: Env, to: Address) {
        let mut token_count: u128 = env.storage().persistent().get(&DataKey::TokenCount).unwrap_or(0);
        assert!(token_count < Self::SUPPLY, "Maximum token supply reached");
        token_count += 1;
        env.storage().persistent().set(&DataKey::TokenCount, &token_count);
        env.storage().persistent().set(&DataKey::Owner(token_count), &to);
        env.events().publish((symbol_short!("Mint"),), (to, token_count));
    }

    pub fn approve(env: Env, owner: Address, to: Address, token_id: u128) {
        owner.require_auth();
        let actual_owner: Address = Self::owner_of(env.clone(), token_id);
        if owner == actual_owner {
            let key: DataKey = DataKey::Approvals(token_id);
            let mut approvals: Vec<Address> = env.storage().persistent().get::<DataKey, Vec<Address>>(&key).unwrap_or_else(|| Vec::new(&env));
            if !approvals.contains(&to) {
                approvals.push_back(to.clone());
                env.storage().persistent().set(&key, &approvals);
                env.events().publish((symbol_short!("Approval"),), (owner, to, token_id));
            }
        } else {
            panic!("Not the token owner");
        }
    }

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, token_id: u128) {
        spender.require_auth();
        let actual_owner: Address = Self::owner_of(env.clone(), token_id);
        if from != actual_owner {
            panic!("From not owner");
        }
        let key: DataKey = DataKey::Approvals(token_id);
        let approvals: Vec<Address> = env.storage().persistent().get::<DataKey, Vec<Address>>(&key).unwrap_or_else(|| Vec::new(&env));
        if !approvals.contains(&spender) {
            panic!("Spender is not approved for this token");
        }
        env.storage().persistent().set(&DataKey::Owner(token_id), &to);
        env.storage().persistent().remove(&DataKey::Approvals(token_id));
        env.events().publish((symbol_short!("Transfer"),), (from, to, token_id));
    }
}

mod test;