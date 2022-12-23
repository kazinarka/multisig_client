use std::rc::Rc;
use clap::ArgMatches;

use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::{read_keypair_file, Signer};
use anchor_client::Cluster;
use anchor_client::Client;

use multisig;
use crate::consts::TRANSACTION_SEED_PREFIX;

/// Call Approve instruction
pub fn approve(matches: &ArgMatches) {
    // get cluster
    let cluster = match matches.value_of("env") {
        Some("dev") => Cluster::Devnet,
        Some("main") => Cluster::Mainnet,
        Some("test") => Cluster::Testnet,
        _ => Cluster::Localnet,
    };

    // get signer wallet
    let wallet_path = matches.value_of("sign").unwrap();
    let wallet_keypair = read_keypair_file(wallet_path).expect("Can't open file-wallet");
    let wallet_pubkey = wallet_keypair.pubkey();

    // connect to anchor client
    let anchor_client = Client::new_with_options(cluster, Rc::new(wallet_keypair), CommitmentConfig::confirmed());
    // get program id
    let program = anchor_client.program(multisig::id());

    // get multisig public key
    let multisig= matches.value_of("multisig").unwrap().parse::<Pubkey>().unwrap();

    // get index of transaction
    let index = matches.value_of("index").unwrap().parse::<u64>().unwrap();

    // find transaction PDA
    let (transaction, _) = Pubkey::find_program_address(&[TRANSACTION_SEED_PREFIX, &multisig.to_bytes(), &index.to_le_bytes()], &multisig::id());

    println!("Index: {:?}", index);
    println!("Transaction: {:?}", transaction);
    println!("Multisig: {:?}", multisig);

    // call instruction
    let signature = program
        .request()
        .accounts(multisig::accounts::Approve {
            multisig: multisig,
            transaction: transaction,
            owner: wallet_pubkey,
        })
        .args(multisig::instruction::Approve {})
        .send().unwrap();

    println!("signature: {:?}", signature);
}