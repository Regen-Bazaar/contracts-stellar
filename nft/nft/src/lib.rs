#![no_std]
mod contract;

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Bytes, Env, Map, String, Vec};

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
    pub verified: bool,
    pub metadata_uri: String
}

#[contracttype]
pub enum DataKey {
    IsPaused,
    Owner(u128),
    TokenCount,
    Approvals(u128),
    BaseURI,
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
        env.storage().instance().set(&DataKey::IsPaused, &false);
        env.storage().instance().set(&DataKey::ADMIN, &admin);
        env.storage().instance().set(&DataKey::MINTER, &admin);
        env.storage().instance().set(&DataKey::VERIFIER, &admin);
        env.storage().instance().set(&DataKey::PAUSER, &admin);

        env.storage().instance().set(&DataKey::BaseURI, &base_token_uri);

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

    pub fn token_uri(env: Env, token_id: u128) -> String {
        let token_uris: Map<u128, String> = env.storage().persistent().get(&DataKey::TokenURIs).expect("should contain token uris");
        let token_uri: String = token_uris.get(token_id).expect("should contain token uri");
        token_uri
    }

    pub fn base_uri(env: Env) -> String {
        let data: String = env.storage().instance().get(&DataKey::BaseURI).expect("should contain uri");
        data
    }

    pub fn pause(env: Env) {
        let pauser: Address = env.storage().instance().get(&DataKey::PAUSER).expect("PAUSER not found");
        pauser.require_auth();
        env.storage().instance().set(&DataKey::IsPaused, &true);
    }

    pub fn unpause(env: Env) {
        let pauser: Address = env.storage().instance().get(&DataKey::PAUSER).expect("PAUSER not found");
        pauser.require_auth();
        env.storage().instance().set(&DataKey::IsPaused, &false);
    }

    pub fn create_impact_product(env: Env, to: Address, impact_data: ImpactData, price: u128) {
        let minter: Address = env.storage().instance().get(&DataKey::MINTER).expect("MINTER not found");
        minter.require_auth();
        let is_paused: bool = env.storage().instance().get(&DataKey::IsPaused).expect("contains value");
        if is_paused {
            panic!("contract paused")
        }
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
            env.storage().persistent().set(&DataKey::CreatorTokens, &creator_tokens_all);
        } else {
            let mut user_tokens_array: Vec<u128> = Vec::new(&env);
            user_tokens_array.push_back(current_id);
            creator_tokens_all.set(to, user_tokens_array);
            env.storage().persistent().set(&DataKey::CreatorTokens, &creator_tokens_all);
        }

        let mut category_tokens_all: Map<String, Vec<u128>> = env.storage().persistent().get(&DataKey::CategoryTokens).expect("Category Tokens Map not found");
        let category_tokens: bool = category_tokens_all.contains_key(impact_data.category.clone());
        if category_tokens {
            let category_tokens_category: Option<Vec<u128>> = category_tokens_all.get(impact_data.category.clone());
            let mut category_tokens_category_array: Vec<u128> = category_tokens_category.unwrap();
            category_tokens_category_array.push_back(current_id);
            category_tokens_all.set(impact_data.category, category_tokens_category_array);
            env.storage().persistent().set(&DataKey::CreatorTokens, &category_tokens_all);
        } else {
            let mut category_tokens_array: Vec<u128> = Vec::new(&env);
            category_tokens_array.push_back(current_id);
            category_tokens_all.set(impact_data.category, category_tokens_array);
            env.storage().persistent().set(&DataKey::CreatorTokens, &category_tokens_all);
        }
    }

    pub fn get_impact_data(env: Env, token_id: u128) -> ImpactData {
        let data: Map<u128, ImpactData> = env.storage().persistent().get(&DataKey::ImpactData).expect("should contain TokenPrices");
        data.get(token_id).expect("No Token Id Found")
    }

    pub fn update_impact_data(env: Env, token_id: u128, new_impact_data: ImpactData) -> bool{
        let owner: Address = Self::owner_of(env.clone(), token_id);
        owner.require_auth();
        let mut impact_data_all: Map<u128, ImpactData> = env.storage().persistent().get(&DataKey::ImpactData).expect("should contain TokenPrices");
        let impact_data_token: ImpactData = impact_data_all.get(token_id).expect("No ImpactData Found");
        
        let old_category: String = impact_data_token.category;
        let new_category: String = new_impact_data.category.clone();
        
        if old_category != new_category {
            Self::remove_from_category(env.clone(), token_id, old_category);
            let mut category_tokens_all: Map<String, Vec<u128>> = env.storage().persistent().get(&DataKey::CategoryTokens).expect("should contain CategoryTokens");
            let mut category_tokens: Vec<u128> = category_tokens_all.get(new_category.clone()).expect("should be an existing category");
            category_tokens.push_back(token_id);

            category_tokens_all.set(new_category, category_tokens);
            env.storage().persistent().set(&DataKey::CategoryTokens, &category_tokens_all);
        }

        impact_data_all.set(token_id, new_impact_data.clone());
        env.storage().persistent().set(&DataKey::ImpactData, &impact_data_all);

        if String::len(&new_impact_data.metadata_uri) > 0 {
            Self::_set_token_uri(env, token_id, new_impact_data.metadata_uri);
        }

        true
    }

    fn remove_from_category(env: Env, token_id:u128, category: String) {
        let data: Map<String, Vec<u128>> = env.storage().persistent().get(&DataKey::CategoryTokens).expect("should contain CategoryTokens");
        let mut category_list: Vec<u128> = data.get(category).expect("No tokens by category found");

        let mut item: u32 = 0;
        for value in category_list.iter(){
            if value == token_id {
                category_list.remove(item);
            }
            item +=1;
        }
    }

    pub fn verify_token(env: Env, token_id: u128, validators: Vec<Address>) -> bool {
        let verifier: Address = env.storage().instance().get(&DataKey::VERIFIER).expect("VERIFIER not found");
        verifier.require_auth();
        let length: u32 = validators.len();
        if length < 5 {
            panic!("From not owner");
        }
        let data: Map<u128, ImpactData> = env.storage().persistent().get(&DataKey::ImpactData).expect("should contain ImpactData");
        let item: ImpactData = data.get(token_id).expect("No item for ImpactData found");
        if item.verified {
            panic!("item already verified");
        }
        true
    }

    pub fn calculate_impact_score(env: Env, token_id: u128) -> u128 {
        let data: Map<u128, ImpactData> = env.storage().persistent().get(&DataKey::ImpactData).expect("should contain ImpactData");
        let item: ImpactData = data.get(token_id).expect("No item for ImpactData found");
        let mut score: u128 = item.impact_value;

        if item.verified {
            score = score * 5 / 10;
        }

        if item.end_date > item.start_date {
            let duration: u128 = item.end_date - item.start_date;
            if duration > (86400 * 30) {
                score = (score * 11) / 10;
            }
            if duration > (86400 * 180) {
                score = (score * 12) / 10;
            }
        }

        score
    }

    pub fn get_token_price(env: Env, token_id: u128) -> u128 {
        let data: Map<u128, u128> = env.storage().persistent().get(&DataKey::TokenPrices).expect("should contain TokenPrices");
        data.get(token_id).expect("No Token Id Found")
    }

    pub fn update_token_price(env: Env, token_id: u128, price: u128){
        let owner: Address = Self::owner_of(env.clone(), token_id);
        owner.require_auth();
        let mut data: Map<u128, u128> = env.storage().persistent().get(&DataKey::TokenPrices).expect("should contain TokenPrices");
        data.set(token_id, price);
        env.storage().persistent().set(&DataKey::TokenPrices, &data);
    }

    pub fn update_token_price_admin(env: Env, token_id: u128, price: u128){
        let admin: Address = env.storage().instance().get(&DataKey::ADMIN).expect("should contain ADMIN");
        admin.require_auth();
        let mut data: Map<u128, u128> = env.storage().persistent().get(&DataKey::TokenPrices).expect("should contain TokenPrices");
        data.set(token_id, price);
        env.storage().persistent().set(&DataKey::TokenPrices, &data);
    }

    pub fn get_tokens_by_creator(env: Env, creator: Address) -> Vec<u128> {
        let data: Map<Address, Vec<u128>> = env.storage().persistent().get(&DataKey::CreatorTokens).expect("should contain CreatorTokens");
        data.get(creator).expect("No tokens by creator found")
    }

    pub fn get_tokens_by_category(env: Env, category: String) -> Vec<u128> {
        let data: Map<String, Vec<u128>> = env.storage().persistent().get(&DataKey::CategoryTokens).expect("should contain CategoryTokens");
        data.get(category).expect("No tokens by category found")
    }

    fn _set_token_uri(env: Env, token_id: u128, token_uri: String) {
        let mut token_uris: Map<u128, String> = env.storage().persistent().get(&DataKey::TokenURIs).expect("should contain TokenURIs");
        token_uris.set(token_id, token_uri);
        env.storage().persistent().set(&DataKey::TokenURIs, &token_uris);
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