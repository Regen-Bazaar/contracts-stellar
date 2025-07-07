use soroban_sdk::{contractclient, Address, Env, String, Vec};
use super::{ImpactData, TokenInfo};

#[contractclient(name = "ImpactProductNFTClient")]
pub trait ImpactProductNFTInterface {
    fn init(env: Env, admin: Address, base_uri: String);
    fn mint(env: Env, to: Address, impact_data: ImpactData, price: u64, royalty_receiver: Address, royalty_bps: u32) -> u32;
    fn get_impact_data(env: Env, token_id: u32) -> Option<ImpactData>;
    fn get_token_info(env: Env, token_id: u32) -> Option<TokenInfo>;
    fn get_tokens_by_creator(env: Env, creator: Address) -> Vec<u32>;
    fn get_tokens_by_category(env: Env, category: String) -> Vec<u32>;
    fn get_all_tokens(env: Env) -> Vec<u32>;
    fn update_impact_data(env: Env, token_id: u32, new_data: ImpactData) -> bool;
    fn update_price(env: Env, token_id: u32, new_price: u64) -> bool;
    fn verify_token(env: Env, token_id: u32) -> bool;
} 