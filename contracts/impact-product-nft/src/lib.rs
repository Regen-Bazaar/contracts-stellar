#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Vec, Map};
use soroban_sdk::contractclient;

#[derive(Clone)]
#[contracttype]
pub struct ImpactData {
    pub category: String,
    pub impact_value: u64,
    pub location: String,
    pub start_date: u64,
    pub end_date: u64,
    pub beneficiaries: String,
    pub verified: bool,
    pub metadata_uri: String,
}

#[derive(Clone)]
#[contracttype]
pub struct RoyaltyInfo {
    pub receiver: Address,
    pub royalty_bps: u32, // basis points (1000 = 10%)
}

#[derive(Clone)]
#[contracttype]
pub struct TokenInfo {
    pub owner: Address,
    pub impact_data: ImpactData,
    pub price: u64,
    pub royalty: RoyaltyInfo,
}

#[contract]
pub struct ImpactProductNFT;

#[contractimpl]
impl ImpactProductNFT {
    pub fn init(env: Env, admin: Address, base_uri: String) {
        let admin_key = symbol_short!("admin");
        let base_uri_key = symbol_short!("base_uri");
        env.storage().instance().set(&admin_key, &admin);
        env.storage().instance().set(&base_uri_key, &base_uri);
        let next_id_key = symbol_short!("next_id");
        env.storage().instance().set(&next_id_key, &0u32);
    }

    pub fn mint(
        env: Env,
        to: Address,
        impact_data: ImpactData,
        price: u64,
        royalty_receiver: Address,
        royalty_bps: u32,
    ) -> u32 {
        let next_id_key = symbol_short!("next_id");
        let token_key = symbol_short!("token");
        let creator_key = symbol_short!("creator");
        let category_key = symbol_short!("cat");
        let all_tokens_key = symbol_short!("all");

        // Get next token ID
        let token_id: u32 = env.storage().instance().get(&next_id_key).unwrap_or(0);
        env.storage().instance().set(&next_id_key, &(token_id + 1));

        // Store token info
        let royalty = RoyaltyInfo { receiver: royalty_receiver.clone(), royalty_bps };
        let info = TokenInfo {
            owner: to.clone(),
            impact_data: impact_data.clone(),
            price,
            royalty,
        };
        env.storage().instance().set(&(token_key.clone(), token_id), &info);

        // Add to creator's list
        let mut creator_tokens: Vec<u32> = env.storage().instance().get(&(creator_key.clone(), to.clone())).unwrap_or(Vec::new(&env));
        creator_tokens.push_back(token_id);
        env.storage().instance().set(&(creator_key.clone(), to.clone()), &creator_tokens);

        // Add to category's list
        let mut cat_tokens: Vec<u32> = env.storage().instance().get(&(category_key.clone(), impact_data.category.clone())).unwrap_or(Vec::new(&env));
        cat_tokens.push_back(token_id);
        env.storage().instance().set(&(category_key.clone(), impact_data.category.clone()), &cat_tokens);

        // Add to all tokens
        let mut all_tokens: Vec<u32> = env.storage().instance().get(&all_tokens_key.clone()).unwrap_or(Vec::new(&env));
        all_tokens.push_back(token_id);
        env.storage().instance().set(&all_tokens_key.clone(), &all_tokens);

        token_id
    }

    pub fn get_impact_data(env: Env, token_id: u32) -> Option<ImpactData> {
        let token_key = symbol_short!("token");
        env.storage().instance().get(&(token_key.clone(), token_id)).map(|info: TokenInfo| info.impact_data)
    }

    pub fn get_token_info(env: Env, token_id: u32) -> Option<TokenInfo> {
        let token_key = symbol_short!("token");
        env.storage().instance().get(&(token_key.clone(), token_id))
    }

    pub fn get_tokens_by_creator(env: Env, creator: Address) -> Vec<u32> {
        let creator_key = symbol_short!("creator");
        env.storage().instance().get(&(creator_key.clone(), creator.clone())).unwrap_or(Vec::new(&env))
    }

    pub fn get_tokens_by_category(env: Env, category: String) -> Vec<u32> {
        let category_key = symbol_short!("cat");
        env.storage().instance().get(&(category_key.clone(), category.clone())).unwrap_or(Vec::new(&env))
    }

    pub fn get_all_tokens(env: Env) -> Vec<u32> {
        let all_tokens_key = symbol_short!("all");
        env.storage().instance().get(&all_tokens_key.clone()).unwrap_or(Vec::new(&env))
    }

    pub fn update_impact_data(env: Env, token_id: u32, new_data: ImpactData) -> bool {
        let token_key = symbol_short!("token");
        if let Some(mut info) = env.storage().instance().get::<_, TokenInfo>(&(token_key.clone(), token_id)) {
            info.impact_data = new_data;
            env.storage().instance().set(&(token_key.clone(), token_id), &info);
            true
        } else {
            false
        }
    }

    pub fn update_price(env: Env, token_id: u32, new_price: u64) -> bool {
        let token_key = symbol_short!("token");
        if let Some(mut info) = env.storage().instance().get::<_, TokenInfo>(&(token_key.clone(), token_id)) {
            info.price = new_price;
            env.storage().instance().set(&(token_key.clone(), token_id), &info);
            true
        } else {
            false
        }
    }

    pub fn verify_token(env: Env, token_id: u32) -> bool {
        let token_key = symbol_short!("token");
        if let Some(mut info) = env.storage().instance().get::<_, TokenInfo>(&(token_key.clone(), token_id)) {
            info.impact_data.verified = true;
            env.storage().instance().set(&(token_key.clone(), token_id), &info);
            true
        } else {
            false
        }
    }
}

mod test; 