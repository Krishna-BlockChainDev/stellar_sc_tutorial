#![cfg(test)]


extern crate std;
use super::*;
use soroban_sdk::{testutils::{Events,Logs}, vec, Env, IntoVal};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
    std::println!("{}", env.logs().all().join("\n"));
    assert_eq!(client.decrement(), 2);
    assert_eq!(client.decrement(), 1);
    std::println!("{}", env.logs().all().join("\n"));
    assert_eq!(client.reset(), 0);
    std::println!("{}", env.logs().all().join("\n"));
    assert_eq!(
        env.events().all(),
        vec![
            &env,
            (
                contract_id.clone(),
                (symbol_short!("COUNTER"), symbol_short!("increment")).into_val(&env),
                1u32.into_val(&env)
            ),
            (
                contract_id.clone(),
                (symbol_short!("COUNTER"), symbol_short!("increment")).into_val(&env),
                2u32.into_val(&env)
            ),
            (
                contract_id.clone(),
                (symbol_short!("COUNTER"), symbol_short!("increment")).into_val(&env),
                3u32.into_val(&env)
            ),
            (
                contract_id.clone(),
                (symbol_short!("COUNTER"), symbol_short!("decrement")).into_val(&env),
                2u32.into_val(&env)
            ),
            (
                contract_id.clone(),
                (symbol_short!("COUNTER"), symbol_short!("decrement")).into_val(&env),
                1u32.into_val(&env)
            ),
            (
                contract_id,
                (symbol_short!("COUNTER"), symbol_short!("reset")).into_val(&env),
                0u32.into_val(&env)
            ),
        ]
    );
    
    
}
