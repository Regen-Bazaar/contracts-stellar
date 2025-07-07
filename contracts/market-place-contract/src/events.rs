use soroban_sdk::{contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NFTCreatedEvent {
    pub nft_id: u64,
    pub creator: Address,
    pub name: String,
    pub price: u128,
    pub token_address: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NFTSoldEvent {
    pub nft_id: u64,
    pub buyer: Address,
    pub previous_owner: Address,
    pub price: u128,
    pub token_address: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NFTPriceUpdatedEvent {
    pub nft_id: u64,
    pub creator: Address,
    pub old_price: u128,
    pub new_price: u128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NFTAvailabilityToggledEvent {
    pub nft_id: u64,
    pub creator: Address,
    pub is_available: bool,
}

pub fn emit_nft_created(
    env: &Env,
    nft_id: u64,
    creator: Address,
    name: String,
    price: u128,
    token_address: Address,
) {
    let event = NFTCreatedEvent {
        nft_id,
        creator,
        name,
        price,
        token_address,
    };
    env.events().publish(("nft_created",), event);
}

pub fn emit_nft_sold(
    env: &Env,
    nft_id: u64,
    buyer: Address,
    previous_owner: Address,
    price: u128,
    token_address: Address,
) {
    let event = NFTSoldEvent {
        nft_id,
        buyer,
        previous_owner,
        price,
        token_address,
    };
    env.events().publish(("nft_sold",), event);
}

pub fn emit_nft_price_updated(
    env: &Env,
    nft_id: u64,
    creator: Address,
    old_price: u128,
    new_price: u128,
) {
    let event = NFTPriceUpdatedEvent {
        nft_id,
        creator,
        old_price,
        new_price,
    };
    env.events().publish(("nft_price_updated",), event);
}

pub fn emit_nft_availability_toggled(
    env: &Env,
    nft_id: u64,
    creator: Address,
    is_available: bool,
) {
    let event = NFTAvailabilityToggledEvent {
        nft_id,
        creator,
        is_available,
    };
    env.events().publish(("nft_availability_toggled",), event);
}
