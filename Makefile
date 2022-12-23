build:
	cargo build

fmt:
	cargo  fmt --all

lint:
	cargo clippy --all && cargo fix --tests --all-features --allow-dirty

pre-commit: build fmt lint

approve_local:
	cargo run -- approve -s /Users/illiafedotov/.config/solana/id.json -m 8on7RnsMroPF79UMdb9VH9tX6KMaSDDFfGZqwr48y5n4 -i 0

approve_dev:
	cargo run -- approve -e dev -s /Users/illiafedotov/.config/solana/id.json -m 8on7RnsMroPF79UMdb9VH9tX6KMaSDDFfGZqwr48y5n4 -i 0

approve_test:
	cargo run -- approve -e testnet -s /Users/illiafedotov/.config/solana/id.json -m 8on7RnsMroPF79UMdb9VH9tX6KMaSDDFfGZqwr48y5n4 -i 0

approve_main:
	cargo run -- approve -e main -s /Users/illiafedotov/.config/solana/id.json -m 8on7RnsMroPF79UMdb9VH9tX6KMaSDDFfGZqwr48y5n4 -i 0

create_multisig_local:
	cargo run -- create_multisig -s /Users/illiafedotov/.config/solana/id.json -o 4kMtMnYWFbsMc7M3jcdnfCceHaiXmrqaMz2QZQAmn88i -t 1

create_multisig_dev:
	cargo run -- create_multisig -e dev -s /Users/illiafedotov/.config/solana/id.json -o 4kMtMnYWFbsMc7M3jcdnfCceHaiXmrqaMz2QZQAmn88i -t 1

create_multisig_test:
	cargo run -- create_multisig -e testnet -s /Users/illiafedotov/.config/solana/id.json -o 4kMtMnYWFbsMc7M3jcdnfCceHaiXmrqaMz2QZQAmn88i -t 1

create_multisig_main:
	cargo run -- create_multisig -e main -s /Users/illiafedotov/.config/solana/id.json -o 4kMtMnYWFbsMc7M3jcdnfCceHaiXmrqaMz2QZQAmn88i -t 1

create_transaction_local:
	cargo run -- create_transaction -s /Users/illiafedotov/.config/solana/id.json -m 8gFGxFUPq2REk5N9Z3mzxErVLaFkjCc6vgedSfiP6ezD

create_transaction_dev:
	cargo run -- create_transaction -e dev -s /Users/illiafedotov/.config/solana/id.json -m 6rdzzuXTTsVCNHSPCeLhBDwedUtHbaJQri1MvLUJiNoa

create_transaction_test:
	cargo run -- create_transaction -e testnet -s /Users/illiafedotov/.config/solana/id.json -m 6rdzzuXTTsVCNHSPCeLhBDwedUtHbaJQri1MvLUJiNoa

create_transaction_main:
	cargo run -- create_transaction -e main -s /Users/illiafedotov/.config/solana/id.json -m 6rdzzuXTTsVCNHSPCeLhBDwedUtHbaJQri1MvLUJiNoa
