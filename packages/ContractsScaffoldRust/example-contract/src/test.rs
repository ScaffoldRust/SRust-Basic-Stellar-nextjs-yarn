#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::Env;

#[test]
fn test_initialize_and_get() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ExampleContract);
    let client = ExampleContractClient::new(&env, &contract_id);

    client.initialize();
    assert_eq!(client.get_count(), 0);
}

#[test]
fn test_increment() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ExampleContract);
    let client = ExampleContractClient::new(&env, &contract_id);

    client.initialize();
    
    // First increment
    assert_eq!(client.increment(), 1);
    assert_eq!(client.get_count(), 1);
    
    // Second increment
    assert_eq!(client.increment(), 2);
    assert_eq!(client.get_count(), 2);
}