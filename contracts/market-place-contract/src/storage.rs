use soroban_sdk::{contracttype, symbol_short, Address, Env, String, Symbol, Vec};

// --- Data Structures ---

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NFTStatus {
    Available,
    Unavailable,
    Sold,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NFT {
    pub id: u64,
    pub creator: Address,
    pub owner: Address,
    pub name: String,
    pub description: String,
    pub category: String,
    pub image_url: String,
    pub price: u128,
    pub token_address: Address,
    pub status: NFTStatus,
    pub created_at: u64,
    pub sold_at: Option<u64>,
}

// --- Storage Keys ---

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StorageKey {
    Admin,
    NFTCounter,
    NFT(u64),
    CreatorNFTs(Address),
    OwnerNFTs(Address),
    CategoryNFTs(String),
    AllNFTs,
}

const NFT_COUNTER: Symbol = symbol_short!("NFT_CTR");

// --- Admin ---

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&StorageKey::Admin)
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&StorageKey::Admin, admin);
}

// --- NFT Counter ---

pub fn get_nft_counter(env: &Env) -> u64 {
    env.storage().instance().get(&NFT_COUNTER).unwrap_or(0)
}

pub fn get_next_nft_id(env: &Env) -> u64 {
    let current = get_nft_counter(env);
    let next = current + 1;
    env.storage().instance().set(&NFT_COUNTER, &next);
    next
}

// --- NFT Storage ---

pub fn get_nft(env: &Env, nft_id: u64) -> Option<NFT> {
    let key = StorageKey::NFT(nft_id);
    env.storage().persistent().get(&key)
}

pub fn set_nft(env: &Env, nft: &NFT) {
    let key = StorageKey::NFT(nft.id);
    env.storage().persistent().set(&key, nft);
}

// --- Indexed Lists ---

pub fn get_creator_nfts(env: &Env, creator: &Address) -> Vec<u64> {
    let key = StorageKey::CreatorNFTs(creator.clone());
    env.storage().persistent().get(&key).unwrap_or(Vec::new(env))
}

pub fn add_creator_nft(env: &Env, creator: &Address, nft_id: u64) {
    let key = StorageKey::CreatorNFTs(creator.clone());
    let mut nfts = get_creator_nfts(env, creator);
    nfts.push_back(nft_id);
    env.storage().persistent().set(&key, &nfts);
}

pub fn get_owner_nfts(env: &Env, owner: &Address) -> Vec<u64> {
    let key = StorageKey::OwnerNFTs(owner.clone());
    env.storage().persistent().get(&key).unwrap_or(Vec::new(env))
}

pub fn add_owner_nft(env: &Env, owner: &Address, nft_id: u64) {
    let key = StorageKey::OwnerNFTs(owner.clone());
    let mut nfts = get_owner_nfts(env, owner);
    nfts.push_back(nft_id);
    env.storage().persistent().set(&key, &nfts);
}

pub fn remove_owner_nft(env: &Env, owner: &Address, nft_id: u64) {
    let key = StorageKey::OwnerNFTs(owner.clone());
    let mut nfts = get_owner_nfts(env, owner);
    if let Some(pos) = nfts.iter().position(|id| id == nft_id) {
        nfts.remove(pos as u32);
    }
    env.storage().persistent().set(&key, &nfts);
}

pub fn get_category_nfts(env: &Env, category: &String) -> Vec<u64> {
    let key = StorageKey::CategoryNFTs(category.clone());
    env.storage().persistent().get(&key).unwrap_or(Vec::new(env))
}

pub fn add_category_nft(env: &Env, category: &String, nft_id: u64) {
    let key = StorageKey::CategoryNFTs(category.clone());
    let mut nfts = get_category_nfts(env, category);
    nfts.push_back(nft_id);
    env.storage().persistent().set(&key, &nfts);
}

pub fn get_all_nft_ids(env: &Env) -> Vec<u64> {
    env.storage()
        .persistent()
        .get(&StorageKey::AllNFTs)
        .unwrap_or(Vec::new(env))
}

pub fn add_all_nft_id(env: &Env, nft_id: u64) {
    let mut all_ids = get_all_nft_ids(env);
    all_ids.push_back(nft_id);
    env.storage().persistent().set(&StorageKey::AllNFTs, &all_ids);
}
