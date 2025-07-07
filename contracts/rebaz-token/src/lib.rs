#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, panic_with_error, contracterror
};

#[derive(Clone)]
#[contracttype]
pub struct StakeInfo {
    pub amount: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub withdrawn: bool,
}

#[derive(Clone)]
#[contracttype]
pub struct GovernanceParams {
    pub proposal_threshold: u64,
    pub voting_period: u64,
    pub voting_delay: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct StakingParams {
    pub min_stake_duration: u64,
    pub max_stake_duration: u64,
    pub base_reward_rate: u64,
}

#[contracterror]
pub enum Error {
    InsufficientBalance = 1,
    OnlyAdminCanMint = 2,
    InsufficientBalanceToBurn = 3,
    CannotStakeZero = 4,
    StakingDurationTooShort = 5,
    StakingDurationTooLong = 6,
    StakeDoesNotExist = 7,
    StakeAlreadyWithdrawn = 8,
    OnlyAdminCanUpdateStakingParams = 9,
    MinDurationGtMaxDuration = 10,
    BaseRewardRateTooHigh = 11,
    OnlyAdminCanUpdateGovernanceParams = 12,
    ContractParamsNotSet = 13,
    ContractPaused = 14,
    OnlyAdminCanPause = 15,
    OnlyAdminCanSlash = 16,
    CannotSlashZero = 17,
    SlashAmountTooHigh = 18,
}

#[contract]
pub struct REBAZToken;

#[contractimpl]
impl REBAZToken {
    // Initialize the contract
    pub fn init(env: Env, admin: Address, initial_supply: u64) {
        let admin_key = symbol_short!("admin");
        let total_supply_key = symbol_short!("total_sup");
        let staking_params_key = symbol_short!("stake_p");
        let governance_params_key = symbol_short!("gov_p");
        let paused_key = symbol_short!("paused");
        
        // Set admin
        env.storage().instance().set(&admin_key, &admin);
        
        // Set initial total supply
        env.storage().instance().set(&total_supply_key, &initial_supply);
        
        // Set initial balances
        let balance_key = symbol_short!("balance");
        env.storage().instance().set(&(balance_key.clone(), admin.clone()), &initial_supply);
        
        // Initialize staking parameters
        let staking_params = StakingParams {
            min_stake_duration: 7 * 24 * 60 * 60, // 7 days in seconds
            max_stake_duration: 365 * 24 * 60 * 60, // 365 days in seconds
            base_reward_rate: 500, // 5% base rate
        };
        env.storage().instance().set(&staking_params_key, &staking_params);
        
        // Initialize governance parameters
        let governance_params = GovernanceParams {
            proposal_threshold: 100000 * 10_u64.pow(7), // 100k tokens with 7 decimals
            voting_period: 40320,
            voting_delay: 11520,
        };
        env.storage().instance().set(&governance_params_key, &governance_params);
        
        // Initialize as not paused
        env.storage().instance().set(&paused_key, &false);
    }
    
    // Check if contract is paused
    fn ensure_not_paused(env: &Env) {
        let paused_key = symbol_short!("paused");
        let paused: bool = env.storage().instance().get(&paused_key).unwrap_or(false);
        if paused {
            panic_with_error!(env, Error::ContractPaused);
        }
    }
    
    // Check if caller is admin
    fn require_admin(env: &Env) {
        let admin_key = symbol_short!("admin");
        let admin: Address = env.storage().instance().get(&admin_key).unwrap();
        admin.require_auth();
    }
    
    // Get total supply
    pub fn total_supply(env: Env) -> u64 {
        let total_supply_key = symbol_short!("total_sup");
        env.storage().instance().get(&total_supply_key).unwrap_or(0)
    }
    
    // Get balance for an address
    pub fn balance(env: Env, owner: Address) -> u64 {
        let balance_key = symbol_short!("balance");
        env.storage().instance().get(&(balance_key.clone(), owner.clone())).unwrap_or(0)
    }
    
    // Transfer tokens
    pub fn transfer(env: Env, from: Address, to: Address, amount: u64) -> bool {
        Self::ensure_not_paused(&env);
        
        if amount == 0 {
            return true;
        }
        
        let balance_key = symbol_short!("balance");
        let from_balance: u64 = env.storage().instance().get(&(balance_key.clone(), from.clone())).unwrap_or(0);
        let to_balance: u64 = env.storage().instance().get(&(balance_key.clone(), to.clone())).unwrap_or(0);
        
        if from_balance < amount {
            panic_with_error!(&env, Error::InsufficientBalance);
        }
        
        env.storage().instance().set(&(balance_key.clone(), from), &(from_balance - amount));
        env.storage().instance().set(&(balance_key, to), &(to_balance + amount));
        
        true
    }
    
    // Mint new tokens (admin only)
    pub fn mint(env: Env, to: Address, amount: u64) -> bool {
        Self::ensure_not_paused(&env);
        Self::require_admin(&env);
        
        let balance_key = symbol_short!("balance");
        let current_balance: u64 = env.storage().instance().get(&(balance_key.clone(), to.clone())).unwrap_or(0);
        let total_supply_key = symbol_short!("total_sup");
        let current_supply: u64 = env.storage().instance().get(&total_supply_key).unwrap_or(0);
        
        env.storage().instance().set(&(balance_key, to), &(current_balance + amount));
        env.storage().instance().set(&total_supply_key, &(current_supply + amount));
        
        true
    }
    
    // Burn tokens
    pub fn burn(env: Env, from: Address, amount: u64) -> bool {
        Self::ensure_not_paused(&env);
        
        let balance_key = symbol_short!("balance");
        let current_balance: u64 = env.storage().instance().get(&(balance_key.clone(), from.clone())).unwrap_or(0);
        let total_supply_key = symbol_short!("total_sup");
        let current_supply: u64 = env.storage().instance().get(&total_supply_key).unwrap_or(0);
        
        if current_balance < amount {
            panic_with_error!(&env, Error::InsufficientBalanceToBurn);
        }
        
        env.storage().instance().set(&(balance_key, from), &(current_balance - amount));
        env.storage().instance().set(&total_supply_key, &(current_supply - amount));
        
        true
    }
    
    // Stake tokens
    pub fn stake(env: Env, staker: Address, amount: u64, duration: u64) -> bool {
        Self::ensure_not_paused(&env);
        
        if amount == 0 {
            panic_with_error!(&env, Error::CannotStakeZero);
        }
        
        let staking_params_key = symbol_short!("stake_p");
        let staking_params: StakingParams = env.storage().instance().get(&staking_params_key).unwrap();
        
        if duration < staking_params.min_stake_duration {
            panic_with_error!(&env, Error::StakingDurationTooShort);
        }
        
        if duration > staking_params.max_stake_duration {
            panic_with_error!(&env, Error::StakingDurationTooLong);
        }
        
        let balance_key = symbol_short!("balance");
        let current_balance: u64 = env.storage().instance().get(&(balance_key.clone(), staker.clone())).unwrap_or(0);
        
        if current_balance < amount {
            panic_with_error!(&env, Error::InsufficientBalance);
        }
        
        // Transfer tokens to contract
        env.storage().instance().set(&(balance_key.clone(), staker.clone()), &(current_balance - amount));
        
        // Create stake info
        let start_time = env.ledger().timestamp();
        let end_time = start_time + duration;
        let stake_info = StakeInfo {
            amount,
            start_time,
            end_time,
            withdrawn: false,
        };
        
        // Store stake info
        let stake_count_key = symbol_short!("stake_cnt");
        let stake_count: u32 = env.storage().instance().get(&(stake_count_key.clone(), staker.clone())).unwrap_or(0);
        let stake_key = symbol_short!("stake");
        env.storage().instance().set(&(stake_key.clone(), staker.clone(), stake_count), &stake_info);
        
        // Update stake count
        env.storage().instance().set(&(stake_count_key, staker.clone()), &(stake_count + 1));
        
        // Update total staked
        let total_staked_key = symbol_short!("total_stk");
        let current_staked: u64 = env.storage().instance().get(&(total_staked_key.clone(), staker.clone())).unwrap_or(0);
        env.storage().instance().set(&(total_staked_key, staker), &(current_staked + amount));
        
        true
    }
    
    // Withdraw staked tokens
    pub fn withdraw(env: Env, staker: Address, stake_id: u32) -> (u64, u64) {
        let stake_key = symbol_short!("stake");
        let stake_info: StakeInfo = env.storage().instance().get(&(stake_key.clone(), staker.clone(), stake_id)).unwrap();
        
        if stake_info.amount == 0 {
            panic_with_error!(&env, Error::StakeDoesNotExist);
        }
        
        if stake_info.withdrawn {
            panic_with_error!(&env, Error::StakeAlreadyWithdrawn);
        }
        
        let balance_key = symbol_short!("balance");
        let current_balance: u64 = env.storage().instance().get(&(balance_key.clone(), staker.clone())).unwrap_or(0);
        
        // Calculate reward
        let reward = Self::calculate_reward(env.clone(), staker.clone(), stake_id);
        
        // Update stake as withdrawn
        let mut updated_stake = stake_info.clone();
        updated_stake.withdrawn = true;
        env.storage().instance().set(&(stake_key, staker.clone(), stake_id), &updated_stake);
        
        // Transfer tokens back to staker
        env.storage().instance().set(&(balance_key.clone(), staker.clone()), &(current_balance + stake_info.amount));
        
        // Update total staked
        let total_staked_key = symbol_short!("total_stk");
        let current_staked: u64 = env.storage().instance().get(&(total_staked_key.clone(), staker.clone())).unwrap_or(0);
        env.storage().instance().set(&(total_staked_key, staker.clone()), &(current_staked - stake_info.amount));
        
        // Mint reward tokens if any
        if reward > 0 {
            let total_supply_key = symbol_short!("total_sup");
            let current_supply: u64 = env.storage().instance().get(&total_supply_key).unwrap_or(0);
            env.storage().instance().set(&(balance_key, staker), &(current_balance + stake_info.amount + reward));
            env.storage().instance().set(&total_supply_key, &(current_supply + reward));
        }
        
        (stake_info.amount, reward)
    }
    
    // Calculate reward for a stake
    pub fn calculate_reward(env: Env, staker: Address, stake_id: u32) -> u64 {
        let stake_key = symbol_short!("stake");
        let stake_info: StakeInfo = env.storage().instance().get(&(stake_key.clone(), staker.clone(), stake_id)).unwrap();
        
        if stake_info.amount == 0 || stake_info.withdrawn {
            return 0;
        }
        
        let current_time = env.ledger().timestamp();
        if current_time < stake_info.end_time {
            return 0; // Not matured yet
        }
        
        let staking_params_key = symbol_short!("stake_p");
        let staking_params: StakingParams = env.storage().instance().get(&staking_params_key).unwrap();
        
        let duration_in_seconds = stake_info.end_time - stake_info.start_time;
        let duration_in_years = (duration_in_seconds * 10000) / (365 * 24 * 60 * 60);
        
        let mut reward_rate = staking_params.base_reward_rate;
        
        // Bonus for longer staking periods
        if duration_in_seconds >= 180 * 24 * 60 * 60 { // 180 days
            reward_rate += 100;
        }
        if duration_in_seconds >= 365 * 24 * 60 * 60 { // 365 days
            reward_rate += 200;
        }
        
        let reward = (stake_info.amount * reward_rate * duration_in_years) / (10000 * 10000);
        reward
    }
    
    // Get stake information
    pub fn get_stake_info(env: Env, user: Address, stake_id: u32) -> (u64, u64, u64, u64) {
        let stake_key = symbol_short!("stake");
        let stake_info: StakeInfo = env.storage().instance().get(&(stake_key.clone(), user.clone(), stake_id)).unwrap();
        let current_reward = if stake_info.withdrawn { 0 } else { Self::calculate_reward(env, user, stake_id) };
        (stake_info.amount, stake_info.start_time, stake_info.end_time, current_reward)
    }
    
    // Get total staked for a user
    pub fn get_total_staked(env: Env, user: Address) -> u64 {
        let total_staked_key = symbol_short!("total_stk");
        env.storage().instance().get(&(total_staked_key.clone(), user.clone())).unwrap_or(0)
    }
    
    // Get voting power (balance + staked)
    pub fn get_voting_power(env: Env, user: Address) -> u64 {
        let balance = Self::balance(env.clone(), user.clone());
        let staked = Self::get_total_staked(env, user);
        balance + staked
    }
    
    // Update staking parameters (admin only)
    pub fn update_staking_params(env: Env, min_duration: u64, max_duration: u64, base_rate: u64) -> bool {
        Self::require_admin(&env);
        
        if min_duration > max_duration {
            panic_with_error!(&env, Error::MinDurationGtMaxDuration);
        }
        
        if base_rate > 5000 {
            panic_with_error!(&env, Error::BaseRewardRateTooHigh);
        }
        
        let staking_params = StakingParams {
            min_stake_duration: min_duration,
            max_stake_duration: max_duration,
            base_reward_rate: base_rate,
        };
        
        let staking_params_key = symbol_short!("stake_p");
        env.storage().instance().set(&staking_params_key, &staking_params);
        
        true
    }
    
    // Update governance parameters (admin only)
    pub fn update_governance_params(env: Env, proposal_threshold: u64, voting_period: u64, voting_delay: u64) -> bool {
        Self::require_admin(&env);
        
        let governance_params = GovernanceParams {
            proposal_threshold,
            voting_period,
            voting_delay,
        };
        
        let governance_params_key = symbol_short!("gov_p");
        env.storage().instance().set(&governance_params_key, &governance_params);
        
        true
    }
    
    // Get staking parameters
    pub fn get_staking_params(env: Env) -> StakingParams {
        let staking_params_key = symbol_short!("stake_p");
        env.storage().instance().get(&staking_params_key).unwrap()
    }
    
    // Get governance parameters
    pub fn get_governance_params(env: Env) -> GovernanceParams {
        let governance_params_key = symbol_short!("gov_p");
        env.storage().instance().get(&governance_params_key).unwrap()
    }
    
    // Get stake count for a user
    pub fn get_stake_count(env: Env, user: Address) -> u32 {
        let stake_count_key = symbol_short!("stake_cnt");
        env.storage().instance().get(&(stake_count_key.clone(), user.clone())).unwrap_or(0)
    }
    
    // Pause the contract (admin only)
    pub fn pause(env: Env) -> bool {
        Self::require_admin(&env);
        
        let paused_key = symbol_short!("paused");
        env.storage().instance().set(&paused_key, &true);
        
        true
    }
    
    // Unpause the contract (admin only)
    pub fn unpause(env: Env) -> bool {
        Self::require_admin(&env);
        
        let paused_key = symbol_short!("paused");
        env.storage().instance().set(&paused_key, &false);
        
        true
    }
    
    // Check if contract is paused
    pub fn is_paused(env: Env) -> bool {
        let paused_key = symbol_short!("paused");
        env.storage().instance().get(&paused_key).unwrap_or(false)
    }
    
    // Slash tokens from a validator (admin only)
    pub fn slash_validator(env: Env, validator: Address, amount: u64) -> bool {
        Self::require_admin(&env);
        
        if amount == 0 {
            panic_with_error!(&env, Error::CannotSlashZero);
        }
        
        let balance_key = symbol_short!("balance");
        let validator_balance: u64 = env.storage().instance().get(&(balance_key.clone(), validator.clone())).unwrap_or(0);
        let validator_staked: u64 = Self::get_total_staked(env.clone(), validator.clone());
        
        if validator_balance + validator_staked < amount {
            panic_with_error!(&env, Error::SlashAmountTooHigh);
        }
        
        let total_supply_key = symbol_short!("total_sup");
        let current_supply: u64 = env.storage().instance().get(&total_supply_key).unwrap_or(0);
        
        // Slash from balance first
        let slash_from_balance = if amount <= validator_balance { amount } else { validator_balance };
        if slash_from_balance > 0 {
            env.storage().instance().set(&(balance_key.clone(), validator.clone()), &(validator_balance - slash_from_balance));
            env.storage().instance().set(&total_supply_key, &(current_supply - slash_from_balance));
        }
        
        // If there's more to slash, slash from staked amounts
        if slash_from_balance < amount {
            let remaining_to_slash = amount - slash_from_balance;
            // For simplicity, we'll slash from the most recent stakes first
            let stake_count = Self::get_stake_count(env.clone(), validator.clone());
            let mut remaining = remaining_to_slash;
            
            for stake_id in (0..stake_count).rev() {
                if remaining == 0 {
                    break;
                }
                
                let stake_key = symbol_short!("stake");
                if let Some(mut stake_info) = env.storage().instance().get::<_, StakeInfo>(&(stake_key.clone(), validator.clone(), stake_id)) {
                    if !stake_info.withdrawn {
                        let slash_from_stake = if remaining <= stake_info.amount { remaining } else { stake_info.amount };
                        stake_info.amount -= slash_from_stake;
                        remaining -= slash_from_stake;
                        
                        if stake_info.amount == 0 {
                            stake_info.withdrawn = true;
                        }
                        
                        env.storage().instance().set(&(stake_key.clone(), validator.clone(), stake_id), &stake_info);
                        
                        // Update total staked
                        let total_staked_key = symbol_short!("total_stk");
                        let current_staked: u64 = env.storage().instance().get(&(total_staked_key.clone(), validator.clone())).unwrap_or(0);
                        env.storage().instance().set(&(total_staked_key, validator.clone()), &(current_staked - slash_from_stake));
                    }
                }
            }
            
            env.storage().instance().set(&total_supply_key, &(current_supply - amount));
        }
        
        true
    }
}

mod test; 