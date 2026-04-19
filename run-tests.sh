#!/usr/bin/bash

export RUST_BACKTRACE=1

# Run normal tests
echo "Running normal tests..."
time cargo test --all-features

# Run examples
echo "Running examples..."
cargo check --examples --all-features
for i in examples/*.rs; do
    echo "Running example: $i"
    NAME=$(basename "$i" .rs)
    time cargo run --example "$NAME" --release --features "full"
done

# Documentation (for coverage)
echo "Running documentation..."
cargo doc --no-deps --document-private-items