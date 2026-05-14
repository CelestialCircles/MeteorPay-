#![cfg(test)]
use super::*;
use soroban_sdk::{Env, Symbol};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, MeteorPayContract);
    let client = MeteorPayContractClient::new(&env, &contract_id);

    let words = client.hello(&Symbol::new(&env, "Dev"));
    assert_eq!(
        words,
        Symbol::new(&env, "hello")
    );
}
