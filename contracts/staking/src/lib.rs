#![no_std]
use soroban_sdk::{
    contract, contractimpl, symbol_short, Address, Env, String, TryFromVal, Val, Vec,
};

mod types;
use types::*;
mod interfaces;
mod test;
use interfaces::{ImpactClient, NftClient, TokenClient};

// Add event types
use soroban_sdk::contracttype;

#[contracttype]
pub struct NFTStakedEvent {
    pub token_id: u32,
    pub owner: Address,
    pub lock_period: u64,
    pub lock_end_time: u64,
    pub multiplier: u32,
}

#[contracttype]
pub struct RewardsClaimedEvent {
    pub token_id: u32,
    pub owner: Address,
    pub amount: u64,
}

#[contracttype]
pub struct NFTUnstakedEvent {
    pub token_id: u32,
    pub owner: Address,
}

#[contracttype]
pub struct StakingParamsUpdatedEvent {
    pub base_reward_rate: u32,
    pub min_lock_period: u64,
    pub max_lock_period: u64,
}

#[contract]
pub struct ImpactProductStaking;

#[contractimpl]
impl ImpactProductStaking {
    pub fn initialize(env: Env, admin: Address, nft_contract: Address, rebaz_token: Address) {
        // Only allow initialization once
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::NFTContract, &nft_contract);
        env.storage()
            .instance()
            .set(&DataKey::REBAZToken, &rebaz_token);
        let params = StakingParams {
            base_reward_rate: 1000,
            min_lock_period: 7 * 24 * 60 * 60,   // 7 days
            max_lock_period: 365 * 24 * 60 * 60, // 365 days
        };
        env.storage().instance().set(&DataKey::Params, &params);
    }

    pub fn stake_nft(env: Env, user: Address, token_id: u32, lock_period: u64) {
        user.require_auth();
        let params: StakingParams = env.storage().instance().get(&DataKey::Params).unwrap();
        if lock_period < params.min_lock_period || lock_period > params.max_lock_period {
            panic!("Invalid lock period");
        }
        if env.storage().instance().has(&DataKey::Stake(token_id)) {
            panic!("Already staked");
        }

        // Get NFT contract address
        let nft_contract: Address = env.storage().instance().get(&DataKey::NFTContract).unwrap();

        // Use proper NFT interface to check ownership
        let nft_client = NftClient::new(&env, &nft_contract);
        let token_id_str = token_id_to_string(&env, token_id);
        let owner = nft_client.owner(&token_id_str);

        if owner != user {
            panic!("Not the token owner");
        }

        // Transfer NFT to this contract using proper interface
        nft_client.transfer(&user, &env.current_contract_address(), &token_id_str);
        let now = env.ledger().timestamp();
        let lock_end_time = now + lock_period;
        let multiplier = calculate_multiplier(lock_period);
        let stake = NFTStake {
            token_id,
            owner: user.clone(),
            start_time: now,
            lock_period,
            lock_end_time,
            last_claim_time: now,
            multiplier,
        };
        env.storage()
            .instance()
            .set(&DataKey::Stake(token_id), &stake);
        let mut staked_tokens: Vec<u32> = env
            .storage()
            .instance()
            .get(&DataKey::StakedTokens(user.clone()))
            .unwrap_or(Vec::new(&env));
        staked_tokens.push_back(token_id);
        env.storage()
            .instance()
            .set(&DataKey::StakedTokens(user.clone()), &staked_tokens);
        // Emit NFTStaked event
        env.events().publish(
            (symbol_short!("nft_stkd"),),
            NFTStakedEvent {
                token_id,
                owner: user,
                lock_period,
                lock_end_time,
                multiplier,
            },
        );
    }

    pub fn claim_rewards(env: Env, user: Address, token_id: u32) -> u64 {
        user.require_auth();
        let mut stake: NFTStake = env
            .storage()
            .instance()
            .get(&DataKey::Stake(token_id))
            .unwrap();
        if stake.owner != user {
            panic!("Not stake owner");
        }
        let reward = calculate_rewards(&env, &stake);
        if reward == 0 {
            panic!("No rewards to claim");
        }
        stake.last_claim_time = env.ledger().timestamp();
        env.storage()
            .instance()
            .set(&DataKey::Stake(token_id), &stake);
        // Cross-contract call to REBAZ token contract to mint reward
        let rebaz_token: Address = env.storage().instance().get(&DataKey::REBAZToken).unwrap();
        let token_client = TokenClient::new(&env, &rebaz_token);
        token_client.mint(&user, &(reward as i128));
        // Emit RewardsClaimed event
        env.events().publish(
            (symbol_short!("r_clmd"),),
            RewardsClaimedEvent {
                token_id,
                owner: user,
                amount: reward,
            },
        );
        reward
    }

    pub fn unstake_nft(env: Env, user: Address, token_id: u32) -> u64 {
        user.require_auth();
        let stake: NFTStake = env
            .storage()
            .instance()
            .get(&DataKey::Stake(token_id))
            .unwrap();
        if stake.owner != user {
            panic!("Not stake owner");
        }
        if env.ledger().timestamp() < stake.lock_end_time {
            panic!("Lock period not ended");
        }
        let reward = Self::claim_rewards(env.clone(), user.clone(), token_id);
        env.storage().instance().remove(&DataKey::Stake(token_id));
        let mut staked_tokens: Vec<u32> = env
            .storage()
            .instance()
            .get(&DataKey::StakedTokens(user.clone()))
            .unwrap_or(Vec::new(&env));
        let mut idx = None;
        for i in 0..staked_tokens.len() {
            if staked_tokens.get(i).unwrap() == token_id {
                idx = Some(i);
                break;
            }
        }
        if let Some(i) = idx {
            staked_tokens.remove_unchecked(i);
        }
        env.storage()
            .instance()
            .set(&DataKey::StakedTokens(user.clone()), &staked_tokens);
        // Cross-contract call to NFT contract to transfer NFT back to user
        let nft_contract: Address = env.storage().instance().get(&DataKey::NFTContract).unwrap();
        let nft_client = NftClient::new(&env, &nft_contract);
        let token_id_str = token_id_to_string(&env, token_id);
        nft_client.transfer(&env.current_contract_address(), &user, &token_id_str);
        // Emit NFTUnstaked event
        env.events().publish(
            (symbol_short!("nft_unstk"),),
            NFTUnstakedEvent {
                token_id,
                owner: user,
            },
        );
        reward
    }

    pub fn get_staked_nfts(env: Env, user: Address) -> Vec<u32> {
        env.storage()
            .instance()
            .get(&DataKey::StakedTokens(user))
            .unwrap_or(Vec::new(&env))
    }

    pub fn get_stake_info(env: Env, token_id: u32) -> Option<NFTStake> {
        env.storage().instance().get(&DataKey::Stake(token_id))
    }

    pub fn pending_rewards(env: Env, token_id: u32) -> u64 {
        if let Some(stake) = env
            .storage()
            .instance()
            .get::<_, NFTStake>(&DataKey::Stake(token_id))
        {
            calculate_rewards(&env, &stake)
        } else {
            0
        }
    }

    pub fn update_staking_params(
        env: Env,
        admin: Address,
        base_reward_rate: u32,
        min_lock_period: u64,
        max_lock_period: u64,
    ) {
        admin.require_auth();
        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if stored_admin != admin {
            panic!("Not admin");
        }
        let params = StakingParams {
            base_reward_rate,
            min_lock_period,
            max_lock_period,
        };
        env.storage().instance().set(&DataKey::Params, &params);
        // Emit StakingParamsUpdated event
        env.events().publish(
            (symbol_short!("paramsupd"),),
            StakingParamsUpdatedEvent {
                base_reward_rate,
                min_lock_period,
                max_lock_period,
            },
        );
    }
}

fn calculate_multiplier(lock_period: u64) -> u32 {
    // Tiers: 30d=1200, 90d=1500, 180d=2000, 365d=3000, else 1000
    if lock_period >= 365 * 24 * 60 * 60 {
        3000
    } else if lock_period >= 180 * 24 * 60 * 60 {
        2000
    } else if lock_period >= 90 * 24 * 60 * 60 {
        1500
    } else if lock_period >= 30 * 24 * 60 * 60 {
        1200
    } else {
        1000
    }
}

fn calculate_rewards(env: &Env, stake: &NFTStake) -> u64 {
    let nft_contract: Address = env.storage().instance().get(&DataKey::NFTContract).unwrap();
    // Use proper impact interface to get impact data
    let impact_client = ImpactClient::new(env, &nft_contract);
    let token_id_str = token_id_to_string(env, stake.token_id);
    let impact_vec: Vec<Val> = impact_client.get_impact_data(&token_id_str);
    let impact_value = u64::try_from_val(env, &impact_vec.get(0).unwrap()).unwrap();
    let verified = bool::try_from_val(env, &impact_vec.get(1).unwrap()).unwrap();
    let mut impact = impact_value;
    if verified {
        impact = (impact * 120) / 100;
    }
    let params: StakingParams = env.storage().instance().get(&DataKey::Params).unwrap();
    let duration = env.ledger().timestamp() - stake.last_claim_time;
    let annual_equiv = (duration * 10000) / 31536000;
    let rewards =
        (impact * params.base_reward_rate as u64 * stake.multiplier as u64 * annual_equiv)
            / (10000 * 10000);
    rewards
}

// Helper function to convert token_id to string
fn token_id_to_string(env: &Env, token_id: u32) -> String {
    // Simple conversion for now - can be improved
    match token_id {
        1 => String::from_str(env, "1"),
        2 => String::from_str(env, "2"),
        3 => String::from_str(env, "3"),
        4 => String::from_str(env, "4"),
        5 => String::from_str(env, "5"),
        _ => String::from_str(env, "1"), // Default fallback
    }
}
