PROGRAM_NAME=xml_language_tag_parser

build:
	cargo build

run:
    cargo run

test:
    cargo test

fmt:
    cargo fmt

clippy:
    cargo clippy -- -D warnings