use clap::ArgMatches;
use std::rc::Rc;

use anchor_client::solana_sdk::commitment_config::CommitmentConfig;
use anchor_client::solana_sdk::pubkey::Pubkey;
use anchor_client::solana_sdk::signature::{read_keypair_file, Signer};
use anchor_client::solana_sdk::system_program;
use anchor_client::Client;
use anchor_client::Cluster;
// use anchor_client::solana_sdk::instruction::{Instruction, AccountMeta};
use anchor_client::anchor_lang::prelude::{AccountMeta, Instructions};
use anchor_client::solana_client::rpc_client::RpcClient;
use anchor_client::solana_sdk::instruction::Instruction;
use anchor_client::solana_sdk::transaction::Transaction;

use multisig;

use crate::consts::TRANSACTION_SEED_PREFIX;
use multisig::state::{TxAccountMeta, TxInstruction};

/// Call CreateTransaction instruction
pub fn create_transaction(matches: &ArgMatches) {
    let url = match matches.value_of("env") {
        Some("main") => "https://api.mainnet-beta.solana.com",
        Some("testnet") => "https://api.testnet.solana.com",
        Some("dev") => "https://api.devnet.solana.com",
        _ => "http://127.0.0.1:8899",
    };

    // get signer wallet
    let wallet_path = matches.value_of("sign").unwrap();
    let wallet_keypair = read_keypair_file(wallet_path).expect("Can't open file-wallet");
    let wallet_pubkey = wallet_keypair.pubkey();

    // connect to client
    let client = RpcClient::new_with_commitment(url.to_string(), CommitmentConfig::confirmed());

    // get program public key
    let program_id = multisig::id();

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

    let instructions = vec![Instruction::new_with_borsh(
        program_id,
        &multisig::instruction::CreateTransaction {
            instructions: vec![TxInstruction {
                program_id: Pubkey::new_unique(),
                keys: vec![
                    TxAccountMeta {
                        pubkey: Pubkey::new_unique(),
                        is_signer: true,
                        is_writable: false,
                    },
                    TxAccountMeta {
                        pubkey: Pubkey::new_unique(),
                        is_signer: false,
                        is_writable: false,
                    },
                    TxAccountMeta {
                        pubkey: Pubkey::new_unique(),
                        is_signer: false,
                        is_writable: false,
                    },
                ],
                data: vec![],
            }],
        },
        vec![
            AccountMeta::new(multisig, false),
            AccountMeta::new(transaction, false),
            AccountMeta::new(wallet_pubkey, true),
            AccountMeta::new(system_program::id(), false),
        ],
    )];
    let mut tx = Transaction::new_with_payer(&instructions, Some(&wallet_pubkey));
    let recent_blockhash = client.get_latest_blockhash().expect("Can't get blockhash");
    tx.sign(&vec![&wallet_keypair], recent_blockhash);
    let id = client.send_transaction(&tx).expect("Transaction failed.");
    println!("tx id: {:?}", id);
}
