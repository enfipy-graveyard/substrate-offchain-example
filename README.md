# Example substrate node with off-chain workers

Example node with off-chain workers.

## Build

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Install required tools:

```bash
./init.sh
```

Build Wasm and native code:

```bash
cargo build --release
```

## Run

### Single node development chain

You can start a development chain with:

```bash
cargo run --release -- --dev
```

Detailed logs may be shown by running the node with the following environment variables set: `RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev`.
