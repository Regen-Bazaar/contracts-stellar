#[cfg(test)]
use crate::{ImpactProductFactory, ImpactProductFactoryClient};
#[cfg(test)]
use soroban_sdk::{Env, Address, String, Vec};
#[cfg(test)]
use soroban_sdk::testutils::{Address as _};

mod contract_nft {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32v1-none/release/nft.wasm"
    );
}

#[test]
fn test_factory() {
    let env: Env = Env::default();
    let admin: Address = Address::generate(&env);

    let contract_id_nft: Address = env.register(contract_nft::WASM, (&admin, "https://ipfs.io/ipfs/QmegWR31kiQcD9S2katTXKxracbAgLs2QLBRGruFW3NhXC"));

    let contract_id: Address = env.register(ImpactProductFactory, (&admin, contract_id_nft));
    let client: ImpactProductFactoryClient<'_> = ImpactProductFactoryClient::new(&env, &contract_id);

    let mut impact_categories: Vec<String> = Vec::new(&env);
    impact_categories.push_back(String::from_str(&env, "Community gardens"));
    impact_categories.push_back(String::from_str(&env, "Tree preservation"));
    impact_categories.push_back(String::from_str(&env, "Eco tourism"));
    impact_categories.push_back(String::from_str(&env, "Educational programs"));
    impact_categories.push_back(String::from_str(&env, "Wildlife Conservation"));
    impact_categories.push_back(String::from_str(&env, "CO2 Emissions Reduction"));
    impact_categories.push_back(String::from_str(&env, "Waste Management"));

    assert_eq!(client.get_supported_categories(), impact_categories)
}