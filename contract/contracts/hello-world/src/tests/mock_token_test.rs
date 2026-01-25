#![cfg(test)]

use crate::mock_token::{MockToken, MockTokenClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_mock_token() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, MockToken);
    let client = MockTokenClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    // Test Initialize
    client.initialize(
        &admin,
        &7,
        &String::from_str(&env, "Mock Token"),
        &String::from_str(&env, "MOCK"),
    );

    assert_eq!(client.decimals(), 7);
    assert_eq!(client.name(), String::from_str(&env, "Mock Token"));
    assert_eq!(client.symbol(), String::from_str(&env, "MOCK"));

    // Test Mint
    client.mint(&user1, &1000);
    assert_eq!(client.balance(&user1), 1000);
    assert_eq!(client.total_supply(), 1000);

    // Test Transfer
    client.transfer(&user1, &user2, &200);
    assert_eq!(client.balance(&user1), 800);
    assert_eq!(client.balance(&user2), 200);
    assert_eq!(client.total_supply(), 1000);

    // Test Insufficient Balance (Expect failure logic would need try_transfer or panic check, 
    // but for simplicity in this basic test we just test happy paths and logic)
    // To test panics we can use verify_panics if desired, but this covers basic happy path.
}
