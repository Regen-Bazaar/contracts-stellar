#![no_std]

mod contract;
mod error;
mod events;
mod storage;
mod test;

use soroban_sdk::{contract, contractimpl, Address, Env, String, Vec, Map};

pub use contract::*;
pub use error::ContractError;
pub use events::*;
pub use storage::{NFT, NFTStatus};


#[contract]
pub struct MarketplaceContract;

#[contractimpl]
impl MarketplaceContract {
    pub fn initialize(env: Env, admin: Address) -> Result<(), ContractError> {
        if storage::has_admin(&env) {
            return Err(ContractError::AlreadyInitialized);
        }
        admin.require_auth();
        storage::set_admin(&env, &admin);
        Ok(())
    }

    /// Creates a new NFT representing a real-world impact product.
    pub fn create_nft(
        env: Env,
        creator: Address,
        name: String,
        description: String,
        category: String,
        image_url: String,
        price: u128,
        token_address: Address,
    ) -> Result<u64, ContractError> {
        creator.require_auth();
        contract::create_nft(
            &env,
            &creator,
            name,
            description,
            category,
            image_url,
            price,
            &token_address,
        )
    }

    /// Buys an NFT from the marketplace.
    pub fn buy_nft(env: Env, buyer: Address, nft_id: u64) -> Result<(), ContractError> {
        buyer.require_auth();
        contract::buy_nft(&env, &buyer, nft_id)
    }

    /// Updates the price of an NFT. Only the creator can do this.
    pub fn update_nft_price(
        env: Env,
        creator: Address,
        nft_id: u64,
        new_price: u128,
    ) -> Result<(), ContractError> {
        creator.require_auth();
        contract::update_nft_price(&env, &creator, nft_id, new_price)
    }

    /// Toggles the availability of an NFT for sale.
    pub fn toggle_nft_availability(
        env: Env,
        creator: Address,
        nft_id: u64,
    ) -> Result<(), ContractError> {
        creator.require_auth();
        contract::toggle_nft_availability(&env, &creator, nft_id)
    }

    // --- Read-Only Functions ---

    /// Retrieves the details of a specific NFT.
    pub fn get_nft(env: Env, nft_id: u64) -> Result<NFT, ContractError> {
        contract::get_nft(&env, nft_id)
    }

    /// Retrieves all NFTs created.
    pub fn get_all_nfts(env: Env) -> Vec<NFT> {
        contract::get_all_nfts(&env)
    }

    /// Retrieves all NFTs within a specific category.
    pub fn get_nfts_by_category(env: Env, category: String) -> Vec<NFT> {
        contract::get_nfts_by_category(&env, category)
    }

    /// Retrieves all NFTs created by a specific user.
    pub fn get_nfts_by_creator(env: Env, creator: Address) -> Vec<NFT> {
        contract::get_nfts_by_creator(&env, creator)
    }

    /// Retrieves all NFTs owned by a specific user.
    pub fn get_owned_nfts(env: Env, owner: Address) -> Vec<NFT> {
        contract::get_owned_nfts(&env, owner)
    }
    
    /// Retrieves statistics about the marketplace.
    pub fn get_marketplace_stats(env: Env) -> Map<String, u64> {
        contract::get_marketplace_stats(&env)
    }
}
