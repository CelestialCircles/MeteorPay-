#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
pub enum DataKey {
    Depositor,
    Recipient,
    Admin,
    Token,
    Amount,
    Claimed,
}

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    /// Lock funds in escrow. Can only be called once.
    pub fn deposit(
        env: Env,
        depositor: Address,
        recipient: Address,
        admin: Address,
        token: Address,
        amount: i128,
    ) {
        depositor.require_auth();
        assert!(amount > 0, "amount must be positive");
        assert!(
            !env.storage().instance().has(&DataKey::Amount),
            "already deposited"
        );

        let client = token::Client::new(&env, &token);
        client.transfer(&depositor, &env.current_contract_address(), &amount);

        env.storage().instance().set(&DataKey::Depositor, &depositor);
        env.storage().instance().set(&DataKey::Recipient, &recipient);
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Amount, &amount);
        env.storage().instance().set(&DataKey::Claimed, &false);
    }

    /// Release escrowed funds to the recipient. Restricted to recipient or admin.
    pub fn claim(env: Env, caller: Address) {
        caller.require_auth();

        let recipient: Address = env.storage().instance().get(&DataKey::Recipient).unwrap();
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        assert!(caller == recipient || caller == admin, "unauthorized");

        let claimed: bool = env.storage().instance().get(&DataKey::Claimed).unwrap();
        assert!(!claimed, "already claimed");

        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let amount: i128 = env.storage().instance().get(&DataKey::Amount).unwrap();

        env.storage().instance().set(&DataKey::Claimed, &true);

        let client = token::Client::new(&env, &token);
        client.transfer(&env.current_contract_address(), &recipient, &amount);
    }

    pub fn get_amount(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Amount).unwrap_or(0)
    }

    pub fn is_claimed(env: Env) -> bool {
        env.storage().instance().get(&DataKey::Claimed).unwrap_or(false)
    }
}

mod test;
