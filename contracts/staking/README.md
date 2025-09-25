# Impact Product Staking Contract

This smart contract enables users to stake impact NFTs and earn rewards based on the NFT's impact metrics and staking duration.

## Overview

The Impact Product Staking contract provides functionality for:

1. Staking impact NFTs for a specified lock period
2. Earning rewards based on impact metrics and staking duration
3. Claiming accumulated rewards
4. Unstaking NFTs after the lock period ends
5. Managing staking parameters (admin only)

## Key Features

### Staking Mechanism
- **Lock Periods**: NFTs can be staked for 7 days to 365 days
- **Multiplier System**: Longer lock periods provide higher reward multipliers
  - 30+ days: 1.2x multiplier
  - 90+ days: 1.5x multiplier  
  - 180+ days: 2.0x multiplier
  - 365+ days: 3.0x multiplier
- **Impact-Based Rewards**: Rewards are calculated based on the NFT's impact value and verification status

### Reward Calculation
Rewards are calculated using the formula:
```
rewards = (impact_value * base_reward_rate * multiplier * duration) / (10000 * 10000)
```

Where:
- `impact_value`: The NFT's impact metric (with 20% bonus if verified)
- `base_reward_rate`: Configurable base rate (default: 1000)
- `multiplier`: Based on lock period tier
- `duration`: Time since last claim in annual equivalent

## Contract Interfaces

### NFT Interface
The contract uses a standard NFT interface for cross-contract calls:

```rust
#[contractclient(name = "NftClient")]
pub trait NftInterface {
    fn owner(env: Env, token_id: String) -> Address;
    fn transfer(env: Env, from: Address, to: Address, token_id: String);
    fn balance(env: Env, owner: Address) -> i128;
    fn mint(env: Env, to: Address, token_id: String);
    fn is_authorized(env: Env, owner: Address, spender: Address, token_id: String) -> bool;
    fn token_metadata(env: Env, token_id: String) -> String;
}
```

### Impact Interface
For NFTs with impact metrics:

```rust
#[contractclient(name = "ImpactClient")]
pub trait ImpactInterface {
    fn get_impact_data(env: Env, token_id: String) -> Vec<Val>;
}
```

### Token Interface
For reward token operations:

```rust
#[contractclient(name = "TokenClient")]
pub trait TokenInterface {
    fn balance(env: Env, owner: Address) -> i128;
    fn transfer(env: Env, from: Address, to: Address, amount: i128);
    fn mint(env: Env, to: Address, amount: i128);
    fn approve(env: Env, from: Address, spender: Address, amount: i128);
    fn allowance(env: Env, owner: Address, spender: Address) -> i128;
}
```

## Functions

### Admin Functions
- `initialize(admin: Address, nft_contract: Address, rebaz_token: Address)`: Initializes the contract
- `update_staking_params(admin: Address, base_reward_rate: u32, min_lock_period: u64, max_lock_period: u64)`: Updates staking parameters

### User Functions
- `stake_nft(user: Address, token_id: u32, lock_period: u64)`: Stakes an NFT for the specified period
- `claim_rewards(user: Address, token_id: u32) -> u64`: Claims accumulated rewards
- `unstake_nft(user: Address, token_id: u32) -> u64`: Unstakes an NFT and claims final rewards

### Query Functions
- `get_staked_nfts(user: Address) -> Vec<u32>`: Returns user's staked NFT IDs
- `get_stake_info(token_id: u32) -> Option<NFTStake>`: Returns stake details
- `pending_rewards(token_id: u32) -> u64`: Returns pending rewards for a token

## Events

The contract emits the following events:
- `NFTStakedEvent`: When an NFT is staked
- `RewardsClaimedEvent`: When rewards are claimed
- `NFTUnstakedEvent`: When an NFT is unstaked
- `StakingParamsUpdatedEvent`: When staking parameters are updated

## Building and Testing

To build the contract:
```bash
cd contracts/staking
cargo build --target wasm32-unknown-unknown --release
```

To run tests:
```bash
cargo test
```

## Integration with Real NFT Contracts

The contract is designed to work with any NFT contract that implements the standard `NftInterface` and `ImpactInterface`. This makes it compatible with:

1. **Standard NFT contracts** that follow the interface
2. **Impact NFT contracts** that include impact metrics
3. **Custom NFT contracts** that implement the required functions

### Required NFT Contract Functions

Your NFT contract must implement:
- `owner(token_id: String) -> Address`: Returns the owner of a token
- `transfer(from: Address, to: Address, token_id: String)`: Transfers a token
- `get_impact_data(token_id: String) -> Vec<Val>`: Returns impact data [impact_value, verified]

### Example Integration

```rust
// In your NFT contract
#[contractimpl]
impl YourNFTContract {
    pub fn owner(env: Env, token_id: String) -> Address {
        // Your ownership logic
    }
    
    pub fn transfer(env: Env, from: Address, to: Address, token_id: String) {
        // Your transfer logic
    }
    
    pub fn get_impact_data(env: Env, token_id: String) -> Vec<Val> {
        // Return [impact_value, verified] as Vec<Val>
        vec![&env, impact_value.into_val(&env), verified.into_val(&env)]
    }
}
```

## Security Considerations

1. **Authorization**: All user functions require proper authorization
2. **Ownership Verification**: NFT ownership is verified before staking
3. **Lock Period Enforcement**: NFTs cannot be unstaked before lock period ends
4. **Admin Controls**: Only admin can update staking parameters
5. **Reward Calculation**: Rewards are calculated based on actual time elapsed

## Future Improvements

1. **Dynamic Multipliers**: Configurable multiplier tiers
2. **Batch Operations**: Stake/unstake multiple NFTs at once
3. **Advanced Rewards**: More sophisticated reward calculation algorithms
4. **Governance**: Community governance for parameter updates 