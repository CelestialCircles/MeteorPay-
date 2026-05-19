#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::Address as _,
    token, Address, Env,
};

fn setup_env() -> (Env, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(MultiAssetRouter, ());
    (env, contract_id)
}

fn create_test_token(env: &Env, admin: &Address) -> Address {
    let token_contract = env.register_stellar_asset_contract_v2(admin.clone());
    token_contract.address()
}

fn mint_token(env: &Env, token: &Address, admin: &Address, recipient: &Address, amount: i128) {
    let token_client = token::StellarAssetClient::new(env, token);
    token_client.mint(recipient, &amount);
}

#[test]
fn test_single_asset_payment() {
    let (env, contract_id) = setup_env();
    let sender = Address::generate(&env);
    let destination = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_test_token(&env, &token_admin);
    
    // Mint tokens to sender
    mint_token(&env, &token, &token_admin, &sender, 1000);
    
    let client = MultiAssetRouterClient::new(&env, &contract_id);
    client.route_payment(&sender, &token, &destination, &500);
    
    let token_client = token::Client::new(&env, &token);
    assert_eq!(token_client.balance(&destination), 500);
    assert_eq!(token_client.balance(&sender), 500);
}

#[test]
fn test_multiple_assets_payment() {
    let (env, contract_id) = setup_env();
    let sender = Address::generate(&env);
    let dest1 = Address::generate(&env);
    let dest2 = Address::generate(&env);
    
    let token_admin1 = Address::generate(&env);
    let token_admin2 = Address::generate(&env);
    let token1 = create_test_token(&env, &token_admin1);
    let token2 = create_test_token(&env, &token_admin2);
    
    // Mint tokens to sender
    mint_token(&env, &token1, &token_admin1, &sender, 1000);
    mint_token(&env, &token2, &token_admin2, &sender, 2000);
    
    let client = MultiAssetRouterClient::new(&env, &contract_id);
    
    let routes = soroban_sdk::vec![
        &env,
        PaymentRoute {
            asset: token1.clone(),
            destination: dest1.clone(),
            amount: 500,
        },
        PaymentRoute {
            asset: token2.clone(),
            destination: dest2.clone(),
            amount: 1000,
        },
    ];
    
    client.route_payments(&sender, &routes);
    
    let token_client1 = token::Client::new(&env, &token1);
    let token_client2 = token::Client::new(&env, &token2);
    
    assert_eq!(token_client1.balance(&dest1), 500);
    assert_eq!(token_client1.balance(&sender), 500);
    assert_eq!(token_client2.balance(&dest2), 1000);
    assert_eq!(token_client2.balance(&sender), 1000);
}

#[test]
fn test_multiple_destinations_same_asset() {
    let (env, contract_id) = setup_env();
    let sender = Address::generate(&env);
    let dest1 = Address::generate(&env);
    let dest2 = Address::generate(&env);
    
    let token_admin = Address::generate(&env);
    let token = create_test_token(&env, &token_admin);
    mint_token(&env, &token, &token_admin, &sender, 1000);
    
    let client = MultiAssetRouterClient::new(&env, &contract_id);
    
    let routes = soroban_sdk::vec![
        &env,
        PaymentRoute {
            asset: token.clone(),
            destination: dest1.clone(),
            amount: 300,
        },
        PaymentRoute {
            asset: token.clone(),
            destination: dest2.clone(),
            amount: 400,
        },
    ];
    
    client.route_payments(&sender, &routes);
    
    let token_client = token::Client::new(&env, &token);
    assert_eq!(token_client.balance(&dest1), 300);
    assert_eq!(token_client.balance(&dest2), 400);
    assert_eq!(token_client.balance(&sender), 300);
}

#[test]
#[should_panic(expected = "invalid routes: empty")]
fn test_empty_routes_fails() {
    let (env, contract_id) = setup_env();
    let sender = Address::generate(&env);
    
    let client = MultiAssetRouterClient::new(&env, &contract_id);
    let routes: soroban_sdk::Vec<PaymentRoute> = soroban_sdk::vec![&env];
    
    client.route_payments(&sender, &routes);
}

#[test]
#[should_panic(expected = "amount must be positive")]
fn test_zero_amount_fails() {
    let (env, contract_id) = setup_env();
    let sender = Address::generate(&env);
    let destination = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_test_token(&env, &token_admin);
    
    mint_token(&env, &token, &token_admin, &sender, 1000);
    
    let client = MultiAssetRouterClient::new(&env, &contract_id);
    client.route_payment(&sender, &token, &destination, &0);
}

#[test]
#[should_panic(expected = "amount must be positive")]
fn test_negative_amount_fails() {
    let (env, contract_id) = setup_env();
    let sender = Address::generate(&env);
    let destination = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_test_token(&env, &token_admin);
    
    mint_token(&env, &token, &token_admin, &sender, 1000);
    
    let client = MultiAssetRouterClient::new(&env, &contract_id);
    client.route_payment(&sender, &token, &destination, &-100);
}

#[test]
#[should_panic(expected = "insufficient balance")]
fn test_insufficient_balance_single_asset() {
    let (env, contract_id) = setup_env();
    let sender = Address::generate(&env);
    let destination = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_test_token(&env, &token_admin);
    
    // Only mint 500 tokens but try to send 1000
    mint_token(&env, &token, &token_admin, &sender, 500);
    
    let client = MultiAssetRouterClient::new(&env, &contract_id);
    client.route_payment(&sender, &token, &destination, &1000);
}

#[test]
#[should_panic(expected = "insufficient balance")]
fn test_insufficient_balance_multi_asset() {
    let (env, contract_id) = setup_env();
    let sender = Address::generate(&env);
    let dest1 = Address::generate(&env);
    let dest2 = Address::generate(&env);
    
    let token_admin1 = Address::generate(&env);
    let token_admin2 = Address::generate(&env);
    let token1 = create_test_token(&env, &token_admin1);
    let token2 = create_test_token(&env, &token_admin2);
    
    // Mint 500 of token1 but try to send 1000
    mint_token(&env, &token1, &token_admin1, &sender, 500);
    mint_token(&env, &token2, &token_admin2, &sender, 2000);
    
    let client = MultiAssetRouterClient::new(&env, &contract_id);
    
    let routes = soroban_sdk::vec![
        &env,
        PaymentRoute {
            asset: token1.clone(),
            destination: dest1.clone(),
            amount: 1000,
        },
        PaymentRoute {
            asset: token2.clone(),
            destination: dest2.clone(),
            amount: 500,
        },
    ];
    
    client.route_payments(&sender, &routes);
}

#[test]
#[should_panic(expected = "insufficient balance")]
fn test_insufficient_balance_multi_route_second_asset() {
    let (env, contract_id) = setup_env();
    let sender = Address::generate(&env);
    let dest1 = Address::generate(&env);
    let dest2 = Address::generate(&env);
    
    let token_admin1 = Address::generate(&env);
    let token_admin2 = Address::generate(&env);
    let token1 = create_test_token(&env, &token_admin1);
    let token2 = create_test_token(&env, &token_admin2);
    
    // Mint enough token1, but insufficient token2
    mint_token(&env, &token1, &token_admin1, &sender, 1000);
    mint_token(&env, &token2, &token_admin2, &sender, 200);
    
    let client = MultiAssetRouterClient::new(&env, &contract_id);
    
    let routes = soroban_sdk::vec![
        &env,
        PaymentRoute {
            asset: token1.clone(),
            destination: dest1.clone(),
            amount: 500,
        },
        PaymentRoute {
            asset: token2.clone(),
            destination: dest2.clone(),
            amount: 1000,
        },
    ];
    
    client.route_payments(&sender, &routes);
}

#[test]
fn test_get_balance() {
    let (env, contract_id) = setup_env();
    let sender = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token = create_test_token(&env, &token_admin);
    
    mint_token(&env, &token, &token_admin, &sender, 1000);
    
    let client = MultiAssetRouterClient::new(&env, &contract_id);
    let balance = client.get_balance(&sender, &token);
    
    assert_eq!(balance, 1000);
}

#[test]
fn test_atomic_execution_all_or_nothing() {
    let (env, contract_id) = setup_env();
    let sender = Address::generate(&env);
    let dest1 = Address::generate(&env);
    let dest2 = Address::generate(&env);
    
    let token_admin1 = Address::generate(&env);
    let token_admin2 = Address::generate(&env);
    let token1 = create_test_token(&env, &token_admin1);
    let token2 = create_test_token(&env, &token_admin2);
    
    mint_token(&env, &token1, &token_admin1, &sender, 1000);
    mint_token(&env, &token2, &token_admin2, &sender, 1000);
    
    let client = MultiAssetRouterClient::new(&env, &contract_id);
    
    // This should succeed and execute all routes atomically
    let routes = soroban_sdk::vec![
        &env,
        PaymentRoute {
            asset: token1.clone(),
            destination: dest1.clone(),
            amount: 250,
        },
        PaymentRoute {
            asset: token2.clone(),
            destination: dest2.clone(),
            amount: 250,
        },
    ];
    
    client.route_payments(&sender, &routes);
    
    let token_client1 = token::Client::new(&env, &token1);
    let token_client2 = token::Client::new(&env, &token2);
    
    // Verify both transfers executed
    assert_eq!(token_client1.balance(&dest1), 250);
    assert_eq!(token_client2.balance(&dest2), 250);
    assert_eq!(token_client1.balance(&sender), 750);
    assert_eq!(token_client2.balance(&sender), 750);
}
