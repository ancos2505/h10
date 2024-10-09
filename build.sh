#!/bin/sh
BIN_NAME=$(awk 'BEGIN{FS="\"" ; found=0} /\[\[bin\]\]/{found=1} /name/{ if(found==1) { print $2 ; found=0 } }' Cargo.toml)
TARGET=$(uname -m)-unknown-linux-musl
cargo build --release --target=${TARGET}
mkdir -p ./dist/
cp -v ./target/${TARGET}/release/${BIN_NAME} ./dist/
