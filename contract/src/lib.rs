#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct MeteorPayContract;

#[contractimpl]
impl MeteorPayContract {
    pub fn hello(env: Env, to: Symbol) -> Symbol {
        Symbol::new(&env, "hello")
    }
}

mod test;
