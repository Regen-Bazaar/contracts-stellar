use soroban_sdk::{contracttype, Address};

#[contracttype]
pub struct NFTStake {
    pub token_id: u32,
    pub owner: Address,
    pub start_time: u64,
    pub lock_period: u64,
    pub lock_end_time: u64,
    pub last_claim_time: u64,
    pub multiplier: u32,
}

#[contracttype]
pub struct StakingParams {
    pub base_reward_rate: u32,
    pub min_lock_period: u64,
    pub max_lock_period: u64,
}

#[contracttype]
pub enum DataKey {
    Stake(u32),            // token_id -> NFTStake
    StakedTokens(Address), // user -> Vec<u32>
    Params,
    Admin,
    NFTContract,
    REBAZToken,
    Owner(u32),  // token_id -> Address
    Impact(u32), // token_id -> Vec<Val>
}
