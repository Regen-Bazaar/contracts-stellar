#[cfg(test)]
use crate::{ImpactProductFactory, ImpactProductFactoryClient};
#[cfg(test)]
use soroban_sdk::{Env, Address, String};
#[cfg(test)]
use soroban_sdk::testutils::{Address as _};

mod contract_nft {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32v1-none/release/nft.wasm"
    );
}

#[test]
fn test_nft() {
    let env: Env = Env::default();
    let admin: Address = Address::generate(&env);

    let contract_id_nft: Address = env.register(contract_nft::WASM, (&admin, "https://ipfs.io/ipfs/QmegWR31kiQcD9S2katTXKxracbAgLs2QLBRGruFW3NhXC"));

    let contract_id: Address = env.register(ImpactProductFactory, (&admin, contract_id_nft));
    let client: ImpactProductFactoryClient<'_> = ImpactProductFactoryClient::new(&env, &contract_id);

    //assert_eq!(client.nft(), String::from_str(&env, "Regen Bazaar Impact Product"))
}