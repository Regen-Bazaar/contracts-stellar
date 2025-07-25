# REBAZ Token Contract

A Stellar smart contract implementation of the REBAZ (Regen Bazaar) token, designed for real-world impact products and governance.

## Features

### Core Token Functionality

- **ERC20-like Token**: Standard token functionality with transfer, mint, and burn capabilities
- **Staking System**: Users can stake tokens for rewards with configurable durations
- **Governance**: Voting power calculation based on token balance and staked amounts
- **Access Control**: Role-based permissions for admin functions

### Staking Features

- **Flexible Duration**: Configurable minimum (7 days) and maximum (365 days) stake periods
- **Reward Calculation**: Dynamic reward rates based on stake duration
- **Multiple Stakes**: Users can have multiple active stakes
- **Early Withdrawal**: Support for early withdrawal with reduced rewards

### Governance Features

- **Voting Power**: Calculated from token balance + staked amounts
- **Proposal Threshold**: Configurable minimum tokens required for proposals
- **Voting Periods**: Configurable voting and delay periods

## Contract Functions

### Initialization

- `init(admin, initial_supply)`: Initialize the contract with admin and initial supply

### Token Operations

- `transfer(from, to, amount)`: Transfer tokens between addresses
- `mint(to, amount)`: Mint new tokens (admin only)
- `burn(from, amount)`: Burn tokens from an address
- `balance(owner)`: Get token balance for an address
- `total_supply()`: Get total token supply

### Staking Operations

- `stake(staker, amount, duration)`: Stake tokens for a specified duration
- `withdraw(staker, stake_id)`: Withdraw staked tokens and receive rewards
- `calculate_reward(staker, stake_id)`: Calculate reward for a stake
- `get_stake_info(user, stake_id)`: Get detailed stake information
- `get_total_staked(user)`: Get total staked amount for a user
- `get_stake_count(user)`: Get number of stakes for a user

### Governance Operations

- `get_voting_power(user)`: Calculate voting power for a user
- `update_staking_params(min_duration, max_duration, base_rate)`: Update staking parameters (admin only)
- `update_governance_params(proposal_threshold, voting_period, voting_delay)`: Update governance parameters (admin only)
- `get_staking_params()`: Get current staking parameters
- `get_governance_params()`: Get current governance parameters

## Building and Testing

### Prerequisites

- Rust toolchain
- Soroban CLI
- Stellar CLI

### Build

```bash
make build
```

### Test

```bash
make test
```

### Clean

```bash
make clean
```

## Deployment

### Testnet Deployment

1. Build the contract:

   ```bash
   make build
   ```

2. Deploy using Stellar CLI:

   ```bash
   stellar contract deploy --wasm optimized_rebaz_token.wasm --network testnet
   ```

3. Initialize the contract:
   ```bash
   stellar contract invoke --id <CONTRACT_ID> --fn init --args <ADMIN_ADDRESS> <INITIAL_SUPPLY> --network testnet
   ```

## Configuration

### Initial Parameters

- **Initial Supply**: 1,000,000 REBAZ tokens (with 7 decimals)
- **Min Stake Duration**: 7 days
- **Max Stake Duration**: 365 days
- **Base Reward Rate**: 5% annually
- **Proposal Threshold**: 100,000 tokens
- **Voting Period**: 40,320 blocks
- **Voting Delay**: 11,520 blocks

### Reward Structure

- **Base Rate**: 5% annually
- **180+ Days**: +1% bonus
- **365+ Days**: +2% bonus

## Security Features

- **Access Control**: Role-based permissions for sensitive operations
- **Reentrancy Protection**: Built-in protection against reentrancy attacks
- **Input Validation**: Comprehensive validation of all inputs
- **Pausable**: Emergency pause functionality (can be added)

## Integration

### Frontend Integration

The contract provides all necessary functions for frontend integration:

1. **Token Management**: Transfer, balance checking, minting
2. **Staking Interface**: Stake creation, withdrawal, reward calculation
3. **Governance**: Voting power calculation, parameter updates
4. **Data Queries**: Comprehensive data retrieval functions

### API Endpoints

For frontend integration, you'll need to call these contract functions:

- `balance(owner)` - Get user token balance
- `get_total_staked(user)` - Get user's staked amount
- `get_voting_power(user)` - Get user's voting power
- `get_stake_info(user, stake_id)` - Get specific stake details
- `calculate_reward(staker, stake_id)` - Calculate potential rewards

## License

MIT License - see LICENSE file for details.

## Support

For questions and support, please refer to the Stellar documentation or contact the development team.
