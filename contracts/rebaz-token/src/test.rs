#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _},
    Address, Env,
};

use crate::REBAZTokenClient;

#[test]
fn test_init() {
    let env = Env::default();
    let contract_id = env.register_contract(None, REBAZToken);
    let client = REBAZTokenClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let initial_supply = 1000000 * 10_u64.pow(7);

    client.init(&admin, &initial_supply);

    assert_eq!(client.total_supply(), initial_supply);
    assert_eq!(client.balance(&admin), initial_supply);
}

#[test]
fn test_transfer() {
    let env = Env::default();
    let contract_id = env.register_contract(None, REBAZToken);
    let client = REBAZTokenClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let initial_supply = 1000000 * 10_u64.pow(7);

    client.init(&admin, &initial_supply);

    let transfer_amount = 1000 * 10_u64.pow(7);
    let result = client.transfer(&admin, &user1, &transfer_amount);
    assert!(result);

    assert_eq!(client.balance(&admin), initial_supply - transfer_amount);
    assert_eq!(client.balance(&user1), transfer_amount);

    let transfer_amount2 = 500 * 10_u64.pow(7);
    let result2 = client.transfer(&user1, &user2, &transfer_amount2);
    assert!(result2);

    assert_eq!(client.balance(&user1), transfer_amount - transfer_amount2);
    assert_eq!(client.balance(&user2), transfer_amount2);
}

#[test]
#[should_panic]
fn test_transfer_insufficient_balance() {
    let env = Env::default();
    let contract_id = env.register_contract(None, REBAZToken);
    let client = REBAZTokenClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);
    let initial_supply = 1000000 * 10_u64.pow(7);

    client.init(&admin, &initial_supply);

    let transfer_amount = 1000 * 10_u64.pow(7);
    client.transfer(&user1, &user2, &transfer_amount);
}

#[test]
fn test_mint() {
    let env = Env::default();
    let contract_id = env.register_contract(None, REBAZToken);
    let client = REBAZTokenClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let initial_supply = 1000000 * 10_u64.pow(7);

    client.init(&admin, &initial_supply);

    env.mock_all_auths();
    let mint_amount = 1000 * 10_u64.pow(7);
    let result = client.mint(&user1, &mint_amount);
    assert!(result);

    assert_eq!(client.total_supply(), initial_supply + mint_amount);
    assert_eq!(client.balance(&user1), mint_amount);
}

#[test]
fn test_burn() {
    let env = Env::default();
    let contract_id = env.register_contract(None, REBAZToken);
    let client = REBAZTokenClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let initial_supply = 1000000 * 10_u64.pow(7);

    client.init(&admin, &initial_supply);

    let burn_amount = 1000 * 10_u64.pow(7);
    let result = client.burn(&admin, &burn_amount);
    assert!(result);

    assert_eq!(client.total_supply(), initial_supply - burn_amount);
    assert_eq!(client.balance(&admin), initial_supply - burn_amount);
}

#[test]
fn test_stake() {
    let env = Env::default();
    let contract_id = env.register_contract(None, REBAZToken);
    let client = REBAZTokenClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let initial_supply = 1000000 * 10_u64.pow(7);

    client.init(&admin, &initial_supply);

    let transfer_amount = 10000 * 10_u64.pow(7);
    client.transfer(&admin, &user1, &transfer_amount);

    let stake_amount = 5000 * 10_u64.pow(7);
    let stake_duration = 7 * 24 * 60 * 60;
    let result = client.stake(&user1, &stake_amount, &stake_duration);
    assert!(result);

    assert_eq!(client.balance(&user1), transfer_amount - stake_amount);
    assert_eq!(client.get_total_staked(&user1), stake_amount);
    assert_eq!(client.get_stake_count(&user1), 1);
}

#[test]
#[should_panic]
fn test_stake_insufficient_balance() {
    let env = Env::default();
    let contract_id = env.register_contract(None, REBAZToken);
    let client = REBAZTokenClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let initial_supply = 1000000 * 10_u64.pow(7);

    client.init(&admin, &initial_supply);

    let stake_amount = 10000 * 10_u64.pow(7);
    let stake_duration = 7 * 24 * 60 * 60;
    client.stake(&user1, &stake_amount, &stake_duration);
}

#[test]
#[should_panic]
fn test_stake_duration_too_short() {
    let env = Env::default();
    let contract_id = env.register_contract(None, REBAZToken);
    let client = REBAZTokenClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let initial_supply = 1000000 * 10_u64.pow(7);

    client.init(&admin, &initial_supply);

    let transfer_amount = 10000 * 10_u64.pow(7);
    client.transfer(&admin, &user1, &transfer_amount);

    let stake_amount = 5000 * 10_u64.pow(7);
    let stake_duration = 1 * 24 * 60 * 60;
    client.stake(&user1, &stake_amount, &stake_duration);
}

#[test]
fn test_withdraw() {
    let env = Env::default();
    let contract_id = env.register_contract(None, REBAZToken);
    let client = REBAZTokenClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let initial_supply = 1000000 * 10_u64.pow(7);

    client.init(&admin, &initial_supply);

    let transfer_amount = 10000 * 10_u64.pow(7);
    client.transfer(&admin, &user1, &transfer_amount);

    let stake_amount = 5000 * 10_u64.pow(7);
    let stake_duration = 7 * 24 * 60 * 60;
    client.stake(&user1, &stake_amount, &stake_duration);

    let (withdrawn_amount, reward) = client.withdraw(&user1, &0);

    assert_eq!(withdrawn_amount, stake_amount);
    assert!(reward >= 0);
    assert_eq!(client.get_total_staked(&user1), 0);
}

#[test]
fn test_calculate_reward() {
    let env = Env::default();
    let contract_id = env.register_contract(None, REBAZToken);
    let client = REBAZTokenClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let initial_supply = 1000000 * 10_u64.pow(7);

    client.init(&admin, &initial_supply);

    let transfer_amount = 10000 * 10_u64.pow(7);
    client.transfer(&admin, &user1, &transfer_amount);

    let stake_amount = 5000 * 10_u64.pow(7);
    let stake_duration = 7 * 24 * 60 * 60;
    client.stake(&user1, &stake_amount, &stake_duration);

    let reward = client.calculate_reward(&user1, &0);
    assert!(reward >= 0);
}

#[test]
fn test_get_voting_power() {
    let env = Env::default();
    let contract_id = env.register_contract(None, REBAZToken);
    let client = REBAZTokenClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let initial_supply = 1000000 * 10_u64.pow(7);

    client.init(&admin, &initial_supply);

    let transfer_amount = 10000 * 10_u64.pow(7);
    client.transfer(&admin, &user1, &transfer_amount);

    let stake_amount = 5000 * 10_u64.pow(7);
    let stake_duration = 7 * 24 * 60 * 60;
    client.stake(&user1, &stake_amount, &stake_duration);

    let voting_power = client.get_voting_power(&user1);
    assert_eq!(voting_power, transfer_amount - stake_amount + stake_amount);
}

#[test]
fn test_update_staking_params() {
    let env = Env::default();
    let contract_id = env.register_contract(None, REBAZToken);
    let client = REBAZTokenClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let initial_supply = 1000000 * 10_u64.pow(7);

    client.init(&admin, &initial_supply);

    env.mock_all_auths();
    let min_duration = 10 * 24 * 60 * 60;
    let max_duration = 400 * 24 * 60 * 60;
    let base_rate = 600;
    let result = client.update_staking_params(&min_duration, &max_duration, &base_rate);
    assert!(result);

    let params = client.get_staking_params();
    assert_eq!(params.min_stake_duration, min_duration);
    assert_eq!(params.max_stake_duration, max_duration);
    assert_eq!(params.base_reward_rate, base_rate);
}

#[test]
fn test_update_governance_params() {
    let env = Env::default();
    let contract_id = env.register_contract(None, REBAZToken);
    let client = REBAZTokenClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    let initial_supply = 1000000 * 10_u64.pow(7);

    client.init(&admin, &initial_supply);

    env.mock_all_auths();
    let proposal_threshold = 200000 * 10_u64.pow(7);
    let voting_period = 50000;
    let voting_delay = 20000;
    let result = client.update_governance_params(&proposal_threshold, &voting_period, &voting_delay);
    assert!(result);

    let params = client.get_governance_params();
    assert_eq!(params.proposal_threshold, proposal_threshold);
    assert_eq!(params.voting_period, voting_period);
    assert_eq!(params.voting_delay, voting_delay);
} 