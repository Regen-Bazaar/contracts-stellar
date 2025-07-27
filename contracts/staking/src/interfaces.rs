use soroban_sdk::{contractclient, Address, Env, String, Val, Vec};

/// Standard NFT interface that follows common NFT contract patterns
#[contractclient(name = "NftClient")]
#[allow(dead_code)]
pub trait NftInterface {
    /// Returns the owner of a specific NFT token
    fn owner(env: Env, token_id: String) -> Address;

    /// Transfers an NFT from one address to another
    /// Requires authorization from the 'from' address
    fn transfer(env: Env, from: Address, to: Address, token_id: String);

    /// Balance of tokens owned by an address
    fn balance(env: Env, owner: Address) -> i128;

    /// Mints an NFT to a specific address
    fn mint(env: Env, to: Address, token_id: String);

    /// Returns true if an address is authorized to manage a specific token
    fn is_authorized(env: Env, owner: Address, spender: Address, token_id: String) -> bool;

    /// Get metadata for a specific token
    fn token_metadata(env: Env, token_id: String) -> String;
}

/// Impact data interface for NFTs that have impact metrics
#[contractclient(name = "ImpactClient")]
#[allow(dead_code)]
pub trait ImpactInterface {
    /// Get impact data for a specific token
    /// Returns a vector containing [impact_value: u64, verified: bool]
    fn get_impact_data(env: Env, token_id: String) -> Vec<Val>;
}

/// Standard token interface for payment tokens
#[contractclient(name = "TokenClient")]
#[allow(dead_code)]
pub trait TokenInterface {
    /// Returns the balance of tokens owned by an address
    fn balance(env: Env, owner: Address) -> i128;

    /// Transfers tokens from one address to another
    fn transfer(env: Env, from: Address, to: Address, amount: i128);

    /// Mints new tokens to a specific address
    fn mint(env: Env, to: Address, amount: i128);

    /// Approves another address to spend tokens on behalf of the owner
    fn approve(env: Env, from: Address, spender: Address, amount: i128);

    /// Returns the allowance of tokens that a spender can use on behalf of the owner
    fn allowance(env: Env, owner: Address, spender: Address) -> i128;
}
