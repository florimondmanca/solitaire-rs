all: build run-tui

build:
	cargo build

run-tui:
	cargo run --bin tui

format:
	cargo fmt
