mod consts;
mod structs;
mod transactions;

use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg, SubCommand,
};

use crate::transactions::approve::approve;
use crate::transactions::create_multisig::create_multisig;
use crate::transactions::create_transaction::create_transaction;

fn main() {
    // match command from command line
    let matches = app_from_crate!()
        .subcommand(
            SubCommand::with_name("create_multisig")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("base")
                        .short("b")
                        .long("base")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("owners")
                        .short("o")
                        .long("owners")
                        .required(false)
                        .takes_value(true)
                        .multiple(true),
                )
                .arg(
                    Arg::with_name("threshold")
                        .short("t")
                        .long("threshold")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("create_transaction")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("index")
                        .short("i")
                        .long("index")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("multisig")
                        .short("m")
                        .long("multisig")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("path")
                        .short("p")
                        .long("path")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("approve")
                .arg(
                    Arg::with_name("sign")
                        .short("s")
                        .long("sign")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("env")
                        .short("e")
                        .long("env")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("multisig")
                        .short("m")
                        .long("multisig")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("index")
                        .short("i")
                        .long("index")
                        .required(false)
                        .takes_value(true),
                ),
        )
        .get_matches();

    // call create_multisig method if matched
    if let Some(matches) = matches.subcommand_matches("create_multisig") {
        create_multisig(matches);
    }

    // call create_transaction method if matched
    if let Some(matches) = matches.subcommand_matches("create_transaction") {
        create_transaction(matches);
    }

    // call approve method if matched
    if let Some(matches) = matches.subcommand_matches("approve") {
        approve(matches);
    }
}
