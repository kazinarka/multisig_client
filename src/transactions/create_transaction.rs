use clap::ArgMatches;
use std::rc::Rc;

use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::{read_keypair_file, Signer};
use anchor_client::solana_sdk::system_program;
use anchor_client::Client;
use anchor_client::Cluster;

use multisig;

use crate::consts::TRANSACTION_SEED_PREFIX;
use crate::structs::{TxAccountMeta, TxInstruction};

/// Call CreateTransaction instruction
pub fn create_transaction(matches: &ArgMatches) {
    // get cluster
    let cluster = match matches.value_of("env") {
        Some("dev") => Cluster::Devnet,
        Some("main") => Cluster::Mainnet,
        Some("testnet") => Cluster::Testnet,
        _ => Cluster::Localnet,
    };

    // get signer wallet
    let wallet_path = matches.value_of("sign").unwrap();
    let wallet_keypair = read_keypair_file(wallet_path).expect("Can't open file-wallet");
    let wallet_pubkey = wallet_keypair.pubkey();

    // connect to anchor client
    let anchor_client = Client::new_with_options(
        cluster,
        Rc::new(wallet_keypair),
        CommitmentConfig::confirmed(),
    );
    // get program public key
    let program = anchor_client.program(multisig::id());

    // try to get index, if no - index = 0
    let index = if let Some(index) = matches.value_of("index") {
        index.parse::<u64>().unwrap()
    } else {
        0_u64
    };

    // get multisig
    let multisig = matches
        .value_of("multisig")
        .unwrap()
        .parse::<Pubkey>()
        .unwrap();

    // find transaction PDA
    let (transaction, _) = Pubkey::find_program_address(
        &[
            TRANSACTION_SEED_PREFIX,
            &multisig.to_bytes(),
            &index.to_le_bytes(),
        ],
        &multisig::id(),
    );

    println!("Multisig: {:?}", multisig);
    println!("Transaction: {:?}", transaction);

    /*
    In this case i can not use TxInstruction and TxAccountMeta structs from multisig::state - because this folder is private.
    I tried to use different approaches with solana_sdk structs like Instruction and AccountMeta, but it didn't work.

    Also as I can see in TS cli app - you used a TransactionInstruction struct from web3 library.
    I didn't find any working analogues in Rust libraries.

    So, as for now - we have a compilation error while calling CreateTransaction instruction.
    The only way that i see now is somehow make this structs public.
     */

    // call instruction
    let signature = program
        .request()
        .accounts(multisig::accounts::CreateTransaction {
            multisig: multisig,
            transaction: transaction,
            proposer: wallet_pubkey,
            system_program: system_program::id(),
        })
        .args(multisig::instruction::CreateTransaction {
            instructions: vec![TxInstruction {
                program_id: Default::default(),
                keys: vec![TxAccountMeta {
                    pubkey: Default::default(),
                    is_signer: false,
                    is_writable: false,
                }],
                data: vec![],
            }],
        })
        .send()
        .unwrap();

    println!("signature: {:?}", signature);
}
