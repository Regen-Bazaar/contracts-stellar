# ImpactProductNFT Contract

A Soroban smart contract for minting, managing, and querying NFTs representing real-world impact products, designed for integration with the REBAZ platform.

## Features

- **NFT Minting**: Mint NFTs with rich impact metadata (category, value, location, dates, beneficiaries, metadata URI, etc.)
- **Ownership Tracking**: Each NFT is owned by an address
- **Metadata & Impact Data**: Store and update impact data for each NFT
- **Enumeration**: List all NFTs, by creator, by category
- **Royalty Info**: Store royalty receiver and basis points for each NFT
- **Price Listing**: Store and update price for each NFT
- **Verification**: Mark NFTs as verified
- **Frontend Integration**: Exposes all necessary read/query functions

## Contract Functions

### Initialization

- `init(admin, base_uri)`: Initialize the contract with admin and base URI

### NFT Operations

- `mint(to, impact_data, price, royalty_receiver, royalty_bps) -> token_id`: Mint a new NFT
- `get_impact_data(token_id) -> ImpactData`: Get impact data for a token
- `get_token_info(token_id) -> TokenInfo`: Get all info for a token
- `get_tokens_by_creator(creator) -> Vec<u32>`: List all tokens by creator
- `get_tokens_by_category(category) -> Vec<u32>`: List all tokens in a category
- `get_all_tokens() -> Vec<u32>`: List all tokens
- `update_impact_data(token_id, new_data)`: Update impact data for a token
- `update_price(token_id, new_price)`: Update price for a token
- `verify_token(token_id)`: Mark a token as verified

## Building and Testing

### Prerequisites

- Rust toolchain
- Soroban CLI

### Build

```bash
cargo build --package impact-product-nft --target wasm32-unknown-unknown --release
```

### Test

```bash
cargo test --package impact-product-nft
```

## Integration

- Designed to work with the REBAZ token and Impact Buyer contracts
- Exposes all necessary functions for frontend and marketplace integration

## License

MIT License
