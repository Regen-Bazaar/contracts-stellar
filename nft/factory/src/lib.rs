#![no_std]
mod contract;

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, String, Vec};

use crate::contract_nft::ImpactData;

mod contract_nft {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32v1-none/release/nft.wasm"
    );
}

#[contract]
pub struct ImpactProductFactory;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImpactProductData {
    pub category: String,
    pub location: String,
    pub start_date: u128,
    pub end_date: u128,
    pub beneficiaries: String,
    pub base_impact_value: u128,
    pub listing_price: u128,
    pub metadata_uri: String
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
struct ImpactParams {
    pub category: String,
    pub base_multiplier: u128,
    pub verified: bool
}

#[contracttype]
pub enum DataKey {
    ADMIN,
    CREATOR,
    VERIFIER,
    IsPaused,
    ImpactProductNft,
    ImpactParameters,
    ImpactCategories
}

#[contractimpl]
impl ImpactProductFactory {
    pub fn __constructor(env: Env, admin: Address, nft_contract: Address) {
        env.storage().instance().set(&DataKey::IsPaused, &false);
        env.storage().instance().set(&DataKey::ADMIN, &admin);
        env.storage().instance().set(&DataKey::CREATOR, &admin);
        env.storage().instance().set(&DataKey::VERIFIER, &admin);

        env.storage().instance().set(&DataKey::ImpactProductNft, &nft_contract);

        let impact_params: Map<String, ImpactParams> = Map::new(&env);
        env.storage().persistent().set(&DataKey::ImpactParameters, &impact_params);

        let impact_categories: Vec<String> = Vec::new(&env);
        env.storage().persistent().set(&DataKey::ImpactCategories, &impact_categories);

        Self::_add_impact_category(env.clone(), String::from_str(&env, "Community gardens"), 1000);
        Self::_add_impact_category(env.clone(), String::from_str(&env, "Tree preservation"), 2500);
        Self::_add_impact_category(env.clone(), String::from_str(&env, "Eco tourism"), 1500);
        Self::_add_impact_category(env.clone(), String::from_str(&env, "Educational programs"), 2000);
        Self::_add_impact_category(env.clone(), String::from_str(&env, "Wildlife Conservation"), 3000);
        Self::_add_impact_category(env.clone(), String::from_str(&env, "CO2 Emissions Reduction"), 3500);
        Self::_add_impact_category(env.clone(), String::from_str(&env, "Waste Management"), 1200);
    }

    pub fn create_impact_product(env: Env, impact_product_data: ImpactProductData) -> u128 {
        let creator: Address = env.storage().instance().get(&DataKey::CREATOR).expect("CREATOR not found");
        creator.require_auth();

        let is_paused: bool = env.storage().instance().get(&DataKey::IsPaused).expect("contains value");
        if is_paused {
            panic!("contract paused")
        }
        if impact_product_data.base_impact_value == 0 {
            panic!("Impact value must be positive")
        }
        if impact_product_data.listing_price == 0 {
            panic!("Price must be positive")
        }
        if String::len(&impact_product_data.category) == 0 {
            panic!("Category cannot be empty")
        }
        if !Self::is_category_supported(env.clone(), impact_product_data.category.clone()) {
            panic!("Unsupported impact category")
        }
        
        let final_impact_value: u128 = Self::calculate_impact_value(env.clone(), impact_product_data.category.clone(), impact_product_data.base_impact_value);
        
        let contract: Address = env.storage().instance().get(&DataKey::ImpactProductNft).expect("Should contain nft address");
        let client: contract_nft::Client<'_> = contract_nft::Client::new(&env, &contract);

        let impact_data: ImpactData = ImpactData { beneficiaries: impact_product_data.beneficiaries, category: impact_product_data.category, end_date: impact_product_data.end_date, impact_value: final_impact_value, location: impact_product_data.location, metadata_uri: impact_product_data.metadata_uri, start_date: impact_product_data.start_date, verified: false };
        let token_id: u128 = client.create_impact_product(&creator, &impact_data, &impact_product_data.listing_price);
        
        token_id
    }

    pub fn verify_impact_product(env: Env, token_id: u128, validators: Vec<Address> ) -> bool {
        let contract: Address = env.storage().instance().get(&DataKey::ImpactProductNft).expect("Should contain nft address");
        let client: contract_nft::Client<'_> = contract_nft::Client::new(&env, &contract);
        client.verify_token(&token_id, &validators)
    }

    pub fn get_supported_categories(env: Env) -> Vec<String> {
        let impact_categories: Vec<String> = env.storage().persistent().get(&DataKey::ImpactCategories).expect("Should contain Impact Categories");
        impact_categories
    }

    fn is_category_supported(env: Env, category: String) -> bool {
        let impact_parameters: Map<String, ImpactParams> = env.storage().persistent().get(&DataKey::ImpactParameters).expect("Should contain Impact Categories");
        let impact_category: ImpactParams = impact_parameters.get(category).expect("should contain category data");
        impact_category.base_multiplier > 0
    }

    pub fn add_impact_category(env: Env, category: String, base_multiplier: u128) {
        let admin: Address = env.storage().instance().get(&DataKey::ADMIN).expect("contains ADMIN");
        admin.require_auth();
        Self::_add_impact_category(env, category, base_multiplier);
    }

    fn _add_impact_category(env: Env, category: String, base_multiplier: u128) {
        if String::len(&category) == 0 {
            panic!("Category cannot be empty")
        }
        if base_multiplier == 0 {
            panic!("Multiplier must be positive")
        }
        if !Self::is_category_supported(env.clone(), category.clone()) {
            panic!("Category already exists")
        }
        
        let mut impact_categories: Vec<String> = env.storage().persistent().get(&DataKey::ImpactCategories).expect("Should contain Impact Categories");
        impact_categories.push_back(category.clone());
        env.storage().persistent().set(&DataKey::ImpactCategories, &impact_categories);

        Self::calculate_and_store_impact_params(env, category, base_multiplier, false);
    }

    pub fn remove_impact_category(env: Env, category: String) {
        let admin: Address = env.storage().instance().get(&DataKey::ADMIN).expect("contains ADMIN");
        admin.require_auth();
        if !Self::is_category_supported(env.clone(), category.clone()) {
            panic!("Category already exists")
        }

        let mut impact_categories: Vec<String> = env.storage().persistent().get(&DataKey::ImpactCategories).expect("Should contain Impact Categories");

        let mut item: u32 = 0;
        for value in impact_categories.iter() {
            if value == category {
                impact_categories.remove(item);
                item -= 1;
            }
            item += 1;
        }
        env.storage().persistent().set(&DataKey::ImpactCategories, &impact_categories);
    }

    pub fn update_impact_params(env: Env, category: String, base_multiplier: u128) {
        let admin: Address = env.storage().instance().get(&DataKey::ADMIN).expect("contains ADMIN");
        admin.require_auth();
        if !Self::is_category_supported(env.clone(), category.clone()) {
            panic!("Category already exists")
        }
        if base_multiplier == 0 {
            panic!("Multiplier must be positive")
        }
        Self::calculate_and_store_impact_params(env, category, base_multiplier, false);
    }

    fn calculate_and_store_impact_params(env: Env, category: String, mult: u128, verified: bool) {
        let mut impact_parameters: Map<String, ImpactParams> = env.storage().persistent().get(&DataKey::ImpactParameters).expect("Should contain Impact Parameters");
        let data: ImpactParams = ImpactParams { category: category.clone(), base_multiplier: mult, verified: verified };
        impact_parameters.set(category, data);
        env.storage().persistent().set(&DataKey::ImpactParameters, &impact_parameters);
    }

    fn calculate_impact_value(env: Env, category: String, base_value: u128) -> u128 {
        if !Self::is_category_supported(env.clone(), category.clone()) {
            panic!("Unsupported impact category")
        }
        let impact_parameters: Map<String, ImpactParams> = env.storage().persistent().get(&DataKey::ImpactParameters).expect("Should contain Impact Parameters");
        let params: ImpactParams = impact_parameters.get(category).expect("should contain category data");
        let calculated_value: u128 = base_value * params.base_multiplier / 10000;
        calculated_value 
    }

    pub fn grant_creator_role(env: Env, creator: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::ADMIN).expect("PAUSER not found");
        admin.require_auth();
        env.storage().instance().set(&DataKey::CREATOR, &creator);
    }

    pub fn pause(env: Env) {
        let admin: Address = env.storage().instance().get(&DataKey::ADMIN).expect("PAUSER not found");
        admin.require_auth();
        env.storage().instance().set(&DataKey::IsPaused, &true);
    }

    pub fn unpause(env: Env) {
        let admin: Address = env.storage().instance().get(&DataKey::ADMIN).expect("PAUSER not found");
        admin.require_auth();
        env.storage().instance().set(&DataKey::IsPaused, &false);
    }
}

mod test;