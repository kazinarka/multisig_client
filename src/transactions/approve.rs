use std::rc::Rc;
use clap::ArgMatches;

use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::{read_keypair_file, Signer};
use anchor_client::Cluster;
use anchor_client::Client;

use multisig;
use crate::consts::TRANSACTION_SEED_PREFIX;

pub fn approve(matches: &ArgMatches) {
    let cluster = match matches.value_of("env") {
        Some("dev") => Cluster::Devnet,
        Some("main") => Cluster::Mainnet,
        Some("test") => Cluster::Testnet,
        _ => Cluster::Localnet,
    };

    let wallet_path = matches.value_of("sign").unwrap();
    let wallet_keypair = read_keypair_file(wallet_path).expect("Can't open file-wallet");
    let wallet_pubkey = wallet_keypair.pubkey();

    let anchor_client = Client::new_with_options(cluster, Rc::new(wallet_keypair), CommitmentConfig::confirmed());
    let program = anchor_client.program(multisig::id());

    let multisig= matches.value_of("multisig").unwrap().parse::<Pubkey>().unwrap();

    let index = matches.value_of("index").unwrap().parse::<u64>().unwrap();

    let (transaction, _) = Pubkey::find_program_address(&[TRANSACTION_SEED_PREFIX, &multisig.to_bytes(), &index.to_le_bytes()], &multisig::id());

    println!("Index: {:?}", index);
    println!("Transaction: {:?}", transaction);
    println!("Multisig: {:?}", multisig);

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