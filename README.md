

# Dex demo build by substrate

This is a decentralized exchange (DEX) build by substrate.

_**WARNING**_ This project is just for demo/poc/self-learning. The author does not guarantee any availability or stability. And we ***DO NOT*** suggest use it in any production environment.

Any questions, feel free to post an issue.

Please refer to the [wiki](https://github.com/alexxuyang/substrate-dex/wiki) for the detail information. (Continuously updated)

## Build

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Install required tools:

```bash
./scripts/init.sh
```

Build Wasm and native code:

```bash
cargo build
```

## Run

### Single node development chain

You can start a development chain with:

```bash
cargo run -- --dev
```

Detailed logs may be shown by running the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev`.

### Play with the dex

Please refer the client/transactions-api to see how to play with the dex chain. You can:

- issue the token
- transfer the token
- create the trade pair
- create the limit order
- cancel the limit order

Also, you can play with it by [polkadotjs](https://github.com/polkadot-js/apps) wallet frontend with:

```bash
git clone https://github.com/polkadot-js/apps
cd apps
yarn
yarn start
```

Then open the link [http://localhost:3000/#/?rpc=ws://127.0.0.1:9944](http://localhost:3000/#/?rpc=ws://127.0.0.1:9944)

### How to test

Test all the test cases by:

```Rust
cargo test -p substrate-dex-runtime -- --test-threads=1 --nocapture
```

Or you can just run one of the test cases by:

```Rust
cargo test [TEST_CASE_NAME] -p substrate-dex-runtime -- --test-threads=1 --nocapture
```

### About next step

- Will provide a full-feature dex front-end build with polkadotjs
- Will upgrade to substrate 2.0 whenever it released
