all: build run-termui

build:
	cargo build

run-termui:
	cargo run --bin termui

format:
	cargo fmt
