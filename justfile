# THIS JUSTFILE REQUIRES NUSHELL TO BE INSTALLED
set shell := ["nu", "-c"]

default:
    @just --list

# build the program
build:
    cargo build

# build the program for release
release:
    cargo build --release

# run the program
run:
    cargo run

# run the program with bunyan tracing
bunyan:
    cargo run --features=bunyan | bunyan
