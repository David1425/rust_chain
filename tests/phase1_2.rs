use rust_chain::blockchain::block::{Block, Transaction};
use rust_chain::blockchain::chain::Chain;
use rust_chain::blockchain::genesis::genesis_block;
use rust_chain::blockchain::state::{State, UTXO};
use rust_chain::wallet::keychain::Wallet;
use rust_chain::wallet::signer::sign_message;
use rust_chain::crypto::keys::generate_keypair;

#[test]
fn test_genesis_block() {
    let genesis = genesis_block();
    assert_eq!(genesis.header.previous_hash, "0");
    assert_eq!(genesis.transactions.len(), 1);
    assert_eq!(genesis.transactions[0].amount, 50);
}

#[test]
fn test_chain_add_block() {
    let mut chain = Chain::new();
    let tx = Transaction {
        from: "alice".to_string(),
        to: "bob".to_string(),
        amount: 10,
        signature: vec![],
    };
    let prev_hash = chain.blocks.last().unwrap().hash.clone();
    let block = Block::new(prev_hash, vec![tx], 1, 12345);
    assert!(chain.add_block(block));
    assert_eq!(chain.blocks.len(), 2);
}

#[test]
fn test_wallet_address_generation() {
    let wallet = Wallet::new();
    assert!(!wallet.address.is_empty());
}

#[test]
fn test_sign_and_verify() {
    let keypair = generate_keypair();
    let message = b"hello blockchain";
    let signature = sign_message(&keypair, message);
    // TODO: Implement signature verification and test it here
    assert_eq!(signature.len(), 64);
}

#[test]
fn test_utxo_state() {
    let mut state = State::new();
    let utxo = UTXO { owner: "alice".to_string(), amount: 100 };
    state.add_utxo("tx1:0".to_string(), utxo.clone());
    assert_eq!(state.get_balance("alice"), 100);
    let spent = state.spend_utxo("tx1:0");
    assert_eq!(spent.unwrap().amount, 100);
    assert_eq!(state.get_balance("alice"), 0);
}
