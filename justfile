# THIS JUSTFILE REQUIRES NUSHELL TO BE INSTALLED
set shell := ["nu", "-c"]

default:
    @just --list

build:
    cargo build

release:
    cargo build --release

run:
    cargo run

bunyan:
    cargo run --features=bunyan | bunyan
