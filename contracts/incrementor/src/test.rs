use crate::{IncrementorContract, IncrementorContractClient};
use soroban_sdk::{testutils::Logs, Env};

extern crate std;

#[test]
fn increment() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementorContract);
    let client = IncrementorContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
    std::println!("{}", env.logs().all().join("\n"));

    assert_eq!(client.decrement(), 2);
    assert_eq!(client.decrement(), 1);
    std::println!("{}", env.logs().all().join("\n"));

    assert_eq!(client.reset(), 0);
    std::println!("{}", env.logs().all().join("\n"));
}