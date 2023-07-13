#!/bin/bash

# Install system dependencies
sudo apt-get update
sudo DEBIAN_FRONTEND=noninteractive apt-get install -y \
    curl \
    unzip \
    python3-pip \
    python3-dev
sudo pip install tiktoken

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh
sh rustup.sh -y
rm rustup.sh

# Install Elixir dependencies
mix local.hex --force
mix local.rebar --force
mix deps.get
