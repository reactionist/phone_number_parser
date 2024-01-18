.PHONY: build run parse parse_local help clean fmt

PHONE_NUMBER ?= 1234567890

build:
    cargo build

run: build
    cargo run

parse: build
    @echo "Running parse command with full format phone number $(PHONE_NUMBER)"
    cargo run -- parse $(PHONE_NUMBER)

parse_local: build
    @echo "Running parse_local command with phone number $(PHONE_NUMBER)"
    cargo run -- parse_local $(PHONE_NUMBER)

help: build
    cargo run -- help

clean:
    cargo clean

fmt:
    cargo fmt
