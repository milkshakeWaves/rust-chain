use rust_chain::core::{Transaction, WalletKeyPair};

#[test]
fn verify_correct_tx_signature() {
    let key_pair = WalletKeyPair::new();

    let mut tx = Transaction::new(
        "from-address".to_string(),
        "to-address".to_string(),
        12345,
        100,
    );
    tx.sign(&key_pair.secret_key);

    assert!(tx.verify_signature(&key_pair.public_key).is_ok());
}

#[test]
fn verify_empty_tx_signature_return_error() {
    let key_pair = WalletKeyPair::new();

    let tx = Transaction::new(
        "from-address".to_string(),
        "to-address".to_string(),
        12345,
        100,
    );

    assert!(tx.verify_signature(&key_pair.public_key).is_err_and(
        |e| e.to_string() == format!("Transaction {} has an empty signature", tx.nonce)
    ));
}
