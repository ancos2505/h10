#!/bin//sh

set -ex

cargo build --release --target=x86_64-unknown-linux-musl
mkdir -p dist/
cp -v ./target/x86_64-unknown-linux-musl/release/h10-server ./dist/
