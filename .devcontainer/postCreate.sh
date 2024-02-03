#!/usr/bin/env bash

# cat > ~/.pgpass <<EOF
# ${PGHOST}:${PGPORT}:${PGDATABASE}:${PGUSER}:${POSTGRES_PASSWORD}
# EOF
# chmod 600 ~/.pgpass

sudo chown vscode:vscode /usr/local/cargo/registry
cargo install cargo-binstall

cargo binstall cargo-edit cargo-watch cargo-expand cargo-deny cargo-udeps nu just

cargo install sqlx-cli --no-default-features --features rustls,postgres

cargo binstall bunyan

#sqlx migrate run