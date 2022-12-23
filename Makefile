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
	cargo run -- approve -e dev -s /Users/illiafedotov/.config/solana/id.json -m 4kMtMnYWFbsMc7M3jcdnfCceHaiXmrqaMz2QZQAmn88i -i 1
