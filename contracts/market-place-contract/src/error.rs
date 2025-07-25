use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    // Initialization
    AlreadyInitialized = 1,

    // Authorization
    AdminOnly = 2,
    CreatorOnly = 3,
    
    // NFT
    NFTNotFound = 4,
    NFTNotAvailable = 5,
    NFTAlreadyOwned = 6,

    // General
    InvalidPrice = 7,
    OperationNotAllowed = 8,
}
