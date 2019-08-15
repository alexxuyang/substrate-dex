#!/usr/bin/env bash

set -e

./scripts/build.sh

cargo build

target/debug/dex --dev --ws-external -ltxpool=trace
