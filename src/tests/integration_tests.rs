use ic_agent::Agent;
use ic_agent::Identity;
use candid::{Encode, Decode};

#[tokio::test]
async fn test_wallet_integration() {
    // Create a local canister
    let url = "http://127.0.0.1:8000";
    let agent = Agent::builder()
        .with_url(url)
        .with_identity(create_test_identity())
        .build()
        .unwrap();

    // Wait for the replica to be ready
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    // Test balance query
    let result = agent
        .query(&principal, "get_balance")
        .with_arg(&Encode!(&principal).unwrap())
        .call()
        .await;

    assert!(result.is_ok());
}

fn create_test_identity() -> impl Identity {
    let rng = ring::rand::SystemRandom::new();
    let key_pair = ring::signature::Ed25519KeyPair::generate_pkcs8(&rng)
        .expect("Could not generate key pair");
    
    ic_agent::identity::BasicIdentity::from_key_pair(
        ring::signature::Ed25519KeyPair::from_pkcs8(key_pair.as_ref())
            .expect("Could not read key pair"),
    )
}