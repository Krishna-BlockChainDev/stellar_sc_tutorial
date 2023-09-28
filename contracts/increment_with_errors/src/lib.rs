#![no_std]
use soroban_sdk::{contract,contracterror, contractimpl, log, symbol_short, Env, Symbol};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    LimitReached = 1,
}
const COUNTER: Symbol = symbol_short!("COUNTER");
const MAX: u32 = 5;
const MIN: u32 = 0;

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment an internal counter; return the new value.
    pub fn increment(env: Env) -> Result<u32, Error>  {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        log!(&env, "count: {}", count);
        count = count + 1;

      
        // Check if the count exceeds the max.
        if count <= MAX {
            // Save the count.
            env.storage().instance().set(&COUNTER, &count);
            // Publish an event about the increment occuring.
            // The event has two topics:
            //   - The "COUNTER" symbol.
            //   - The "increment" symbol.
            // The event data is the count.
            // env.events()
            //     .publish((COUNTER, symbol_short!("increment")), count);
            //env.storage().instance().bump(100,500);
            
            // Return the count to the caller.
            Ok(count)
        } else {
            // Return an error if the max is exceeded.
            Err(Error::LimitReached)
        }
    }      



    // decrement an internal counter; return the new value.
    pub fn decrement(env: Env) -> Result<u32, Error>  {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        log!(&env, "count sould be greator than 0 : {}", count);
        if count > MIN {
        count = count - 1;
        env.storage().instance().set(&COUNTER, &count);
        // env.events()
        // .publish((COUNTER, symbol_short!("decrement")), count);
        //env.storage().instance().bump(100,500);
        Ok(count)
        }
        else{
             // Return an error if the count is 0 it can not be decrement with 0.
             Err(Error::LimitReached)
        }
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