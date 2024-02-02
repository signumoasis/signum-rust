# THIS JUSTFILE REQUIRES NUSHELL TO BE INSTALLED
set shell := ["nu", "-c"]

default:
    @just --list

# build the program
build:
    cargo build

# run the program with bunyan tracing
bunyan:
    cargo run --features=bunyan | bunyan

# build the program for release
release:
    cargo build --release

# run the program
run:
    cargo run

# run cargo nextest
test:
    cargo nextest run
# cargo watch with default tracing (tracing-subscriber)
watch:
    cargo watch -x check -x "run"

# cargo watch with bunyan tracing
watch-bunyan:
    cargo watch -x check -x "run --features=bunyan | bunyan"
