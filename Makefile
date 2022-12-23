build:
	cargo build

fmt:
	cargo  fmt --all

lint:
	cargo clippy --all && cargo fix --tests --all-features --allow-dirty

pre-commit: build fmt lint test

approve:
	cargo run -- approve -e dev -s /Users/illiafedotov/.config/solana/id.json -m 8on7RnsMroPF79UMdb9VH9tX6KMaSDDFfGZqwr48y5n4 -i 0

create_multisig:
	cargo run -- create_multisig -e dev -s /Users/illiafedotov/.config/solana/id.json -o 4kMtMnYWFbsMc7M3jcdnfCceHaiXmrqaMz2QZQAmn88i -t 1

create_transaction:
	cargo run -- create_transaction -e dev -s /Users/illiafedotov/.config/solana/id.json -m 6rdzzuXTTsVCNHSPCeLhBDwedUtHbaJQri1MvLUJiNoa
