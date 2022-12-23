build:
	cd program; cargo build-bpf

fmt:
	cd program; cargo  fmt --all

lint:
	cd program; cargo clippy --all && cargo fix --tests --all-features --allow-dirty

test-say-hello:
	cd program; cargo test-bpf --test say_hello

test: test-say-hello

pre-commit: build fmt lint test

deploy:
	sh deploy.sh

approve:
	cargo run -- approve -e dev -s /Users/illiafedotov/.config/solana/id.json -m 8on7RnsMroPF79UMdb9VH9tX6KMaSDDFfGZqwr48y5n4 -i 1

create_multisig:
	cargo run -- create_multisig -e dev -s /Users/illiafedotov/.config/solana/id.json -b 4kMtMnYWFbsMc7M3jcdnfCceHaiXmrqaMz2QZQAmn88i -o 4kMtMnYWFbsMc7M3jcdnfCceHaiXmrqaMz2QZQAmn88i -t 1
