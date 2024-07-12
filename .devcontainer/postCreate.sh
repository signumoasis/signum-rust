#!/usr/bin/env bash

sudo apt-get update
sudo apt-get install -y clang lldb lld

rustup component add rust-analyzer

# cat > ~/.pgpass <<EOF
# ${PGHOST}:${PGPORT}:${PGDATABASE}:${PGUSER}:${POSTGRES_PASSWORD}
# EOF
# chmod 600 ~/.pgpass

#sudo chown vscode:vscode /usr/local/cargo/registry
# cargo --color never install cargo-binstall
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

cargo --color never binstall  -y cargo-edit cargo-watch cargo-expand cargo-deny cargo-udeps nu just

cargo --color never binstall -y bunyan

cargo --color never install sqlx-cli --no-default-features --features rustls,sqlite,postgres


#sqlx migrate run
