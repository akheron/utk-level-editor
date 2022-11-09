#!/bin/sh

set -e
cargo build --target wasm32-unknown-unknown
wasm-bindgen ../target/wasm32-unknown-unknown/debug/utk_level_editor_web.wasm --out-dir dist --out-name index --target web

#wasm-opt -Oz -o dist/index_bg.wasm dist/index_bg.wasm
