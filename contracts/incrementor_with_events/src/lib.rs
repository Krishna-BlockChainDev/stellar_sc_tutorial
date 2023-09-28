#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment an internal counter; return the new value.
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        count = count + 1;

        log!(&env, "testing event count: {}", count);
        env.storage().instance().set(&COUNTER, &count);
        // Publish an event about the increment occuring.
        // The event has two topics:
        //   - The "COUNTER" symbol.
        //   - The "increment" symbol.
        // The event data is the count.
        env.events()
            .publish((COUNTER, symbol_short!("increment")), count);
        env.storage().instance().bump(100,500);
        count
    }


    // decrement an internal counter; return the new value.
    pub fn decrement(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
                     count = count - 1;

        log!(&env, "count: {}", count);

        env.storage().instance().set(&COUNTER, &count);
        env.events()
        .publish((COUNTER, symbol_short!("decrement")), count);
        env.storage().instance().bump(100,500);
        count
    }

    // Reset an internal counter; return the new value.
    pub fn reset(env: Env) -> u32 {
        let count: u32 = 0;

        log!(&env, "count: {}", count);
        env.storage().instance().set(&COUNTER, &count);
        env.events()
        .publish((COUNTER, symbol_short!("reset")), count);
        env.storage().instance().bump(100,500);
        count
    }
}

#[cfg(test)]
mod test;