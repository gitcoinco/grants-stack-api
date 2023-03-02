#!/bin/bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add Rust to the PATH
source $HOME/.cargo/env

# Install libpq-dev
sudo apt-get update
sudo apt-get install -y libpq-dev build-essential

# Install diesel_cli with the postgres feature
cargo install diesel_cli --no-default-features --features postgres
