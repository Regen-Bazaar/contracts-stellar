use crate::error::ContractError;
use crate::events::*;
use crate::storage::{self, *};
use soroban_sdk::{token, Address, Env, String, Vec, Map};

/// Creates a new NFT, assigning it to the creator and storing its metadata.
pub fn create_nft(
    env: &Env,
    creator: &Address,
    name: String,
    description: String,
    category: String,
    image_url: String,
    price: u128,
    token_address: &Address,
) -> Result<u64, ContractError> {
    if price == 0 {
        return Err(ContractError::InvalidPrice);
    }

    let nft_id = get_next_nft_id(env);
    let timestamp = env.ledger().timestamp();

    let nft = NFT {
        id: nft_id,
        creator: creator.clone(),
        owner: creator.clone(),
        name,
        description,
        category: category.clone(),
        image_url,
        price,
        token_address: token_address.clone(),
        status: NFTStatus::Available,
        created_at: timestamp,
        sold_at: None,
    };

    set_nft(env, &nft);
    add_creator_nft(env, creator, nft_id);
    add_owner_nft(env, creator, nft_id);
    add_category_nft(env, &category, nft_id);
    add_all_nft_id(env, nft_id);

    emit_nft_created(
        env,
        nft_id,
        creator.clone(),
        nft.name.clone(),
        nft.price,
        nft.token_address.clone(),
    );

    Ok(nft_id)
}


/// Allows a user to buy an NFT that is listed for sale.
pub fn buy_nft(
    env: &Env,
    buyer: &Address,
    nft_id: u64,
) -> Result<(), ContractError> {
    let mut nft = get_nft(env, nft_id)?;
    
    if nft.status != NFTStatus::Available {
        return Err(ContractError::NFTNotAvailable);
    }
    
    if nft.owner == *buyer {
        return Err(ContractError::NFTAlreadyOwned);
    }
    
    let token_client = token::Client::new(env, &nft.token_address);
    token_client.transfer(buyer, &nft.owner, &(nft.price as i128));
    
    let previous_owner = nft.owner.clone();
    remove_owner_nft(env, &previous_owner, nft_id);
    add_owner_nft(env, buyer, nft_id);
    
    nft.owner = buyer.clone();
    nft.status = NFTStatus::Sold;
    nft.sold_at = Some(env.ledger().timestamp());
    
    set_nft(env, &nft);
    
    emit_nft_sold(
        env,
        nft_id,
        buyer.clone(),
        previous_owner,
        nft.price,
        nft.token_address.clone(),
    );
    
    Ok(())
}

/// Allows the creator of an NFT to update its price.
pub fn update_nft_price(
    env: &Env,
    creator: &Address,
    nft_id: u64,
    new_price: u128,
) -> Result<(), ContractError> {
    let mut nft = get_nft(env, nft_id)?;
    
    if nft.creator != *creator {
        return Err(ContractError::CreatorOnly);
    }
    
    if new_price == 0 {
        return Err(ContractError::InvalidPrice);
    }
    
    let old_price = nft.price;
    nft.price = new_price;
    
    set_nft(env, &nft);
    
    emit_nft_price_updated(env, nft_id, creator.clone(), old_price, new_price);
    
    Ok(())
}

/// Allows the creator to toggle an NFT's availability for sale.
pub fn toggle_nft_availability(
    env: &Env,
    creator: &Address,
    nft_id: u64,
) -> Result<(), ContractError> {
    let mut nft = get_nft(env, nft_id)?;
    
    if nft.creator != *creator {
        return Err(ContractError::CreatorOnly);
    }
    
    let new_status = match nft.status {
        NFTStatus::Available => NFTStatus::Unavailable,
        NFTStatus::Unavailable => NFTStatus::Available,
        NFTStatus::Sold => return Err(ContractError::OperationNotAllowed),
    };
    
    nft.status = new_status.clone();
    set_nft(env, &nft);
    
    let is_available = matches!(new_status, NFTStatus::Available);
    
    emit_nft_availability_toggled(env, nft_id, creator.clone(), is_available);
    
    Ok(())
}

// --- Read-Only Functions for Frontend ---

/// Gets the details for a single NFT.
pub fn get_nft(env: &Env, nft_id: u64) -> Result<NFT, ContractError> {
    storage::get_nft(env, nft_id).ok_or(ContractError::NFTNotFound)
}

/// Gets a list of all NFTs ever created.
pub fn get_all_nfts(env: &Env) -> Vec<NFT> {
    let nft_ids = get_all_nft_ids(env);
    let mut nfts = Vec::new(env);
    
    for nft_id in nft_ids.iter() {
        if let Ok(nft) = get_nft(env, nft_id) {
            nfts.push_back(nft);
        }
    }
    
    nfts
}

/// Gets all NFTs belonging to a specific category.
pub fn get_nfts_by_category(env: &Env, category: String) -> Vec<NFT> {
    let nft_ids = get_category_nfts(env, &category);
    let mut nfts = Vec::new(env);
    
    for nft_id in nft_ids.iter() {
        if let Ok(nft) = get_nft(env, nft_id) {
            nfts.push_back(nft);
        }
    }
    
    nfts
}

/// Gets all NFTs created by a specific address.
pub fn get_nfts_by_creator(env: &Env, creator: Address) -> Vec<NFT> {
    let nft_ids = get_creator_nfts(env, &creator);
    let mut nfts = Vec::new(env);
    
    for nft_id in nft_ids.iter() {
        if let Ok(nft) = get_nft(env, nft_id) {
            nfts.push_back(nft);
        }
    }
    
    nfts
}

/// Gets all NFTs currently owned by a specific address.
pub fn get_owned_nfts(env: &Env, owner: Address) -> Vec<NFT> {
    let nft_ids = get_owner_nfts(env, &owner);
    let mut nfts = Vec::new(env);
    
    for nft_id in nft_ids.iter() {
        if let Ok(nft) = get_nft(env, nft_id) {
            nfts.push_back(nft);
        }
    }
    
    nfts
}

/// Provides statistics about the marketplace.
pub fn get_marketplace_stats(env: &Env) -> Map<String, u64> {
    let mut stats = Map::new(env);
    
    let total_nfts = get_nft_counter(env);
    stats.set(String::from_str(env, "total_nfts"), total_nfts);
    
    let all_nfts = get_all_nfts(env);
    let mut available_count = 0u64;
    let mut sold_count = 0u64;
    
    for nft in all_nfts.iter() {
        match nft.status {
            NFTStatus::Available => available_count += 1,
            NFTStatus::Sold => sold_count += 1,
            _ => {}
        }
    }
    
    stats.set(String::from_str(env, "available_nfts"), available_count);
    stats.set(String::from_str(env, "sold_nfts"), sold_count);
    
    stats
}
