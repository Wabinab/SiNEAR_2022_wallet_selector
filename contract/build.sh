#!/bin/bash
set -e

export CONTRACT=contract.wasm

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/$CONTRACT ./res/
wasm-opt -Os -o res/output_s.wasm res/$CONTRACT
wasm-opt -Oz -o res/output.wasm res/$CONTRACT
ls res -lh