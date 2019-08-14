#!/usr/bin/env bash

set -e

./scripts/build.sh

cargo build

cargo run -- purge-chain --dev -y
cargo run -- --dev
