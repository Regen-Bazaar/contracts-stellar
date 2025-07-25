#![no_std]
mod contract;

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Bytes, Env, String, Vec, U256};

#[contract]
pub struct ImpactProductNFT;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImpacttData {
    pub category: String,
    pub impact_value: U256,
    pub location: String,
    pub start_date: U256,
    pub end_date: U256,
    pub beneficiaries: String,
    pub verfied: bool,
    pub metadata_uri: String
}

#[contracttype]
pub enum DataKey {
    Owner(i128),
    TokenCount,
    Approvals(i128),
    BASEURI,
    ADMIN,
    MINTER,
    VERIFIER,
    PAUSER
}

#[contractimpl]
impl ImpactProductNFT {
    const SUPPLY: i128 = 1000;
    const NAME: &'static str = "Regen Bazaar Impact Product";
    const SYMBOL: &'static str = "RIP";

    //const PLATFORM_FEE_BPS: i128 = 1000;

    pub fn __constructor(env: Env, admin: Address, base_token_uri: String) {
        env.storage().instance().set(&DataKey::ADMIN, &admin);
        env.storage().instance().set(&DataKey::MINTER, &admin);
        env.storage().instance().set(&DataKey::VERIFIER, &admin);
        env.storage().instance().set(&DataKey::PAUSER, &admin);

        env.storage().instance().set(&DataKey::BASEURI, &base_token_uri);
    }

    pub fn owner_of(env: Env, token_id: i128) -> Address {
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
        let data: String = env.storage().instance().get(&DataKey::BASEURI).expect("");
        data
    }

    pub fn is_approved(env: Env, operator: Address, token_id: i128) -> bool {
        let key: DataKey = DataKey::Approvals(token_id);
        let approvals: Vec<Address> = env.storage().persistent().get::<DataKey, Vec<Address>>(&key).unwrap_or_else(|| Vec::new(&env));
        approvals.contains(&operator)
    }

    pub fn transfer(env: Env, owner: Address, to: Address, token_id: i128) {
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

    pub fn mint(env: Env, to: Address) {
        let mut token_count: i128 = env.storage().persistent().get(&DataKey::TokenCount).unwrap_or(0);
        assert!(token_count < Self::SUPPLY, "Maximum token supply reached");
        token_count += 1;
        env.storage().persistent().set(&DataKey::TokenCount, &token_count);
        env.storage().persistent().set(&DataKey::Owner(token_count), &to);
        env.events().publish((symbol_short!("Mint"),), (to, token_count));
    }

    pub fn approve(env: Env, owner: Address, to: Address, token_id: i128) {
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

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, token_id: i128) {
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