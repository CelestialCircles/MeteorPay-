#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, Vec};

/// Represents a payment route with asset and destination
#[contracttype]
pub struct PaymentRoute {
    pub asset: Address,      // Token contract address (SAC)
    pub destination: Address, // Destination address for this asset
    pub amount: i128,         // Amount to transfer
}

#[contract]
pub struct MultiAssetRouter;

#[contractimpl]
impl MultiAssetRouter {
    /// Route multiple assets to their respective destinations in one atomic transaction.
    /// 
    /// Args:
    ///   - env: Environment
    ///   - sender: Address initiating the payment (must authorize)
    ///   - routes: Vector of PaymentRoute structs specifying asset, destination, and amount
    ///
    /// Errors:
    ///   - "sender must authorize" if sender doesn't authorize
    ///   - "invalid routes: empty" if routes vector is empty
    ///   - "amount must be positive" if any amount is <= 0
    ///   - "insufficient balance" if sender doesn't have enough balance for any asset
    pub fn route_payments(env: Env, sender: Address, routes: Vec<PaymentRoute>) {
        sender.require_auth();
        assert!(!routes.is_empty(), "invalid routes: empty");

        let mut totals: Vec<(Address, i128)> = Vec::new(&env);

        // First pass: validate routes and accumulate amounts by asset.
        for i in 0..routes.len() {
            let route = &routes.get_unchecked(i);
            assert!(route.amount > 0, "amount must be positive");

            let mut found = false;
            for j in 0..totals.len() {
                let mut entry = totals.get_unchecked(j).clone();
                if entry.0 == route.asset {
                    entry.1 += route.amount;
                    totals.set_unchecked(j, entry);
                    found = true;
                    break;
                }
            }
            if !found {
                totals.push_back((route.asset.clone(), route.amount));
            }
        }

        // Second pass: ensure sender has enough balance for each asset total.
        for i in 0..totals.len() {
            let (asset, amount) = totals.get_unchecked(i);
            let token_client = token::Client::new(&env, &asset);
            let balance = token_client.balance(&sender);
            assert!(balance >= amount, "insufficient balance");
        }

        // Final pass: execute all transfers atomically.
        for i in 0..routes.len() {
            let route = &routes.get_unchecked(i);
            let token_client = token::Client::new(&env, &route.asset);
            token_client.transfer(&sender, &route.destination, &route.amount);
        }
    }

    /// Simplified routing for a single asset payment
    ///
    /// Args:
    ///   - env: Environment
    ///   - sender: Address initiating the payment (must authorize)
    ///   - asset: Token contract address
    ///   - destination: Destination address
    ///   - amount: Amount to transfer
    ///
    /// Errors:
    ///   - "sender must authorize" if sender doesn't authorize
    ///   - "amount must be positive" if amount is <= 0
    ///   - "insufficient balance" if sender doesn't have enough balance
    pub fn route_payment(
        env: Env,
        sender: Address,
        asset: Address,
        destination: Address,
        amount: i128,
    ) {
        sender.require_auth();
        assert!(amount > 0, "amount must be positive");

        let token_client = token::Client::new(&env, &asset);
        let balance = token_client.balance(&sender);
        assert!(balance >= amount, "insufficient balance");

        token_client.transfer(&sender, &destination, &amount);
    }

    /// Get the balance of a sender for a specific asset
    ///
    /// Args:
    ///   - env: Environment
    ///   - sender: Address to check balance for
    ///   - asset: Token contract address
    ///
    /// Returns: Balance amount
    pub fn get_balance(env: Env, sender: Address, asset: Address) -> i128 {
        let token_client = token::Client::new(&env, &asset);
        token_client.balance(&sender)
    }
}

mod test;
