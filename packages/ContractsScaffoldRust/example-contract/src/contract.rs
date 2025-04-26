use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct ExampleContract;

#[contractimpl]
impl ExampleContract {
    /// Initializes the contract with a counter set to 0.
    pub fn initialize(env: Env) {
        env.storage().instance().set(&COUNTER, &0i32);
    }

    /// Increments the counter by 1 and returns the new value.
    pub fn increment(env: Env) -> i32 {
        let current = env.storage().instance().get(&COUNTER).unwrap_or(0i32);
        let new_value = current + 1;
        env.storage().instance().set(&COUNTER, &new_value);
        new_value
    }

    /// Returns the current value of the counter.
    pub fn get_count(env: Env) -> i32 {
        env.storage().instance().get(&COUNTER).unwrap_or(0i32)
    }
}