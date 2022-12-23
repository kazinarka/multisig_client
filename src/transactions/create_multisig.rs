use std::rc::Rc;
use clap::ArgMatches;

use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::{read_keypair_file, Signer};
use anchor_client::solana_sdk::signer::keypair::Keypair;
use anchor_client::Cluster;
use anchor_client::Client;

use multisig;
use crate::consts::MULTISIG_SEED_PREFIX;

pub fn create_multisig(matches: &ArgMatches) {
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

    let base = if let Some(base) = matches.value_of("base") {
        base.parse::<Pubkey>().unwrap()
    } else {
        Keypair::new().pubkey()
    };

    let (multisig, _)= Pubkey::find_program_address(&[MULTISIG_SEED_PREFIX, &base.to_bytes()], &multisig::id());

    let threshold = matches.value_of("threshold").unwrap().parse::<u8>().unwrap();

    let owners = matches.values_of("owners").unwrap()
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|value| value.parse::<Pubkey>().unwrap())
        .collect();

    println!("Owners: {:?}", owners);
    println!("Threshold: {:?}", threshold);
    println!("Base: {:?}", base);
    println!("Multisig: {:?}", multisig);

    let signature = program
        .request()
        .accounts(multisig::accounts::CreateMultisig {
            multisig: multisig,
            payer: wallet_pubkey,
            system_program: anchor_client::solana_sdk::system_program::id(),
        })
        .args(multisig::instruction::CreateMultisig {
            base: base.to_bytes(),
            owners: owners,
            threshold: threshold,
        })
        .send().unwrap();

    println!("signature: {:?}", signature);
}