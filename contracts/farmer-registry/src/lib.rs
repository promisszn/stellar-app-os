#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, BytesN, Env, String,
};

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct FarmerProfile {
    pub wallet_address: Address,
    pub land_doc_hash: BytesN<32>,
    pub region_geohash: String,
    pub registered_at: u64,
}

#[contract]
pub struct FarmerRegistry;

#[contractimpl]
impl FarmerRegistry {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&symbol_short!("ADMIN")) {
            panic!("already initialized");
        }
        env.storage().instance().set(&symbol_short!("ADMIN"), &admin);
    }

    pub fn register_farmer(
        env: Env,
        wallet_address: Address,
        land_doc_hash: BytesN<32>,
        region_geohash: String,
    ) -> FarmerProfile {
        wallet_address.require_auth();

        if env.storage().persistent().has(&wallet_address) {
            panic!("farmer already registered");
        }

        let profile = FarmerProfile {
            wallet_address: wallet_address.clone(),
            land_doc_hash,
            region_geohash,
            registered_at: env.ledger().timestamp(),
        };

        env.storage()
            .persistent()
            .set(&wallet_address, &profile);

        env.events().publish(
            (symbol_short!("FarmerReg"), wallet_address.clone()),
            profile.clone(),
        );

        profile
    }

    pub fn get_farmer(env: Env, wallet_address: Address) -> Option<FarmerProfile> {
        env.storage().persistent().get(&wallet_address)
    }

    pub fn is_registered(env: Env, wallet_address: Address) -> bool {
        env.storage().persistent().has(&wallet_address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, String};

    fn setup() -> (Env, Address, FarmerRegistryClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, FarmerRegistry);
        let client = FarmerRegistryClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        client.initialize(&admin);
        (env, admin, client)
    }

    #[test]
    fn test_register_farmer() {
        let (env, _, client) = setup();
        let farmer = Address::generate(&env);
        let land_hash = BytesN::from_array(&env, &[1u8; 32]);
        let geohash = String::from_str(&env, "s00000000000");

        let profile = client.register_farmer(&farmer, &land_hash, &geohash);

        assert_eq!(profile.wallet_address, farmer);
        assert_eq!(profile.land_doc_hash, land_hash);
        assert_eq!(profile.region_geohash, geohash);
        assert!(profile.registered_at > 0);
    }

    #[test]
    fn test_get_farmer() {
        let (env, _, client) = setup();
        let farmer = Address::generate(&env);
        let land_hash = BytesN::from_array(&env, &[1u8; 32]);
        let geohash = String::from_str(&env, "s00000000000");

        client.register_farmer(&farmer, &land_hash, &geohash);

        let retrieved = client.get_farmer(&farmer).unwrap();
        assert_eq!(retrieved.wallet_address, farmer);
    }

    #[test]
    fn test_is_registered() {
        let (env, _, client) = setup();
        let farmer = Address::generate(&env);
        let land_hash = BytesN::from_array(&env, &[1u8; 32]);
        let geohash = String::from_str(&env, "s00000000000");

        assert!(!client.is_registered(&farmer));
        client.register_farmer(&farmer, &land_hash, &geohash);
        assert!(client.is_registered(&farmer));
    }

    #[test]
    #[should_panic(expected = "farmer already registered")]
    fn test_double_registration_rejected() {
        let (env, _, client) = setup();
        let farmer = Address::generate(&env);
        let land_hash = BytesN::from_array(&env, &[1u8; 32]);
        let geohash = String::from_str(&env, "s00000000000");

        client.register_farmer(&farmer, &land_hash, &geohash);
        client.register_farmer(&farmer, &land_hash, &geohash);
    }
}
