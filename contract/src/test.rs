#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    token, Address, Env, IntoVal,
};

fn setup() -> (Env, Address, Address, Address, Address, Address, i128) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let depositor = Address::generate(&env);
    let recipient = Address::generate(&env);

    // Deploy a token contract for testing
    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_id = token_contract.address();
    let token_client = token::StellarAssetClient::new(&env, &token_id);
    let amount: i128 = 1000;
    token_client.mint(&depositor, &amount);

    let contract_id = env.register_contract(None, EscrowContract);

    (env, contract_id, depositor, recipient, admin, token_id, amount)
}

#[test]
fn test_deposit_and_claim_by_recipient() {
    let (env, contract_id, depositor, recipient, admin, token_id, amount) = setup();
    let client = EscrowContractClient::new(&env, &contract_id);

    client.deposit(&depositor, &recipient, &admin, &token_id, &amount);

    assert_eq!(client.get_amount(), amount);
    assert!(!client.is_claimed());

    client.claim(&recipient);

    assert!(client.is_claimed());

    let token_client = token::Client::new(&env, &token_id);
    assert_eq!(token_client.balance(&recipient), amount);
}

#[test]
fn test_claim_by_admin() {
    let (env, contract_id, depositor, recipient, admin, token_id, amount) = setup();
    let client = EscrowContractClient::new(&env, &contract_id);

    client.deposit(&depositor, &recipient, &admin, &token_id, &amount);
    client.claim(&admin);

    assert!(client.is_claimed());
    let token_client = token::Client::new(&env, &token_id);
    assert_eq!(token_client.balance(&recipient), amount);
}

#[test]
#[should_panic(expected = "unauthorized")]
fn test_claim_by_unauthorized_fails() {
    let (env, contract_id, depositor, recipient, admin, token_id, amount) = setup();
    let client = EscrowContractClient::new(&env, &contract_id);

    client.deposit(&depositor, &recipient, &admin, &token_id, &amount);

    let stranger = Address::generate(&env);
    client.claim(&stranger);
}

#[test]
#[should_panic(expected = "already claimed")]
fn test_double_claim_fails() {
    let (env, contract_id, depositor, recipient, admin, token_id, amount) = setup();
    let client = EscrowContractClient::new(&env, &contract_id);

    client.deposit(&depositor, &recipient, &admin, &token_id, &amount);
    client.claim(&recipient);
    client.claim(&recipient);
}

#[test]
#[should_panic(expected = "already deposited")]
fn test_double_deposit_fails() {
    let (env, contract_id, depositor, recipient, admin, token_id, amount) = setup();
    let client = EscrowContractClient::new(&env, &contract_id);

    client.deposit(&depositor, &recipient, &admin, &token_id, &amount);
    client.deposit(&depositor, &recipient, &admin, &token_id, &amount);
}

#[test]
#[should_panic(expected = "amount must be positive")]
fn test_zero_amount_fails() {
    let (env, contract_id, depositor, recipient, admin, token_id, _) = setup();
    let client = EscrowContractClient::new(&env, &contract_id);

    client.deposit(&depositor, &recipient, &admin, &token_id, &0);
}
