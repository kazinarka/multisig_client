# Rust client for multisig

Cli application that provides user to call instructions on Multisig smart contract

# Set up

Override toolchain

> rustup override set nightly-2022-12-20

Build

> make build

# Commands

- You can use Makefile commands to run application.
But you need to change some fields in commands.


- Or you can run application by yourself. Here is the commands:

### Approve

> cargo run -- approve -e <cluster> -s <key path> -m <multisig> -i <index>

### Create Multisig

> cargo run -- create_multisig -e <cluster> -s <key path> -o <list of owners> -t <threshold> -b <base>

### Create Transaction (not completed yet)

> cargo run -- create_transaction -e <cluster> -s <key path> -m <multisig> -i <index> -p <path>

# Useful notes

cluster - can be:

- dev - Devnet
- main - Mainnet
- testnet - Testnet
- Localnet (just a blank space)

key path - path to your private key file (.json)

multisig - pubkey of multisig

index - index of transactions amount

list of owners - pubkeys divided by SPACE

threshold - threshold number (should be greater than 0 and lower tham owners amount)

base - can be any pubkey or nothing (in this case it will be generated randomly)

path - path to JSON file with instructions data (not completed yet)