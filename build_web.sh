#!/bin/bash

# Requirements:
#   wasm-bindgen
#   wasm-opt

cargo build --release --target wasm32-unknown-unknown

if [ $? -eq 0 ]; then
    echo "cargo build successful"

    wasm-bindgen --no-typescript --target web --out-dir ./docs/ --out-name "rust_tower_defence" ./target/wasm32-unknown-unknown/release/rust_tower_defence.wasm

    if [ $? -eq 0 ]; then
        echo "wasm-bindgen successful"

        wasm-opt -Oz -o docs/rust_tower_defence_bg.wasm docs/rust_tower_defence_bg.wasm
        
        if [ $? -eq 0 ]; then
            echo "wasm-opt successful"
            echo "Build successful!"
        else
            echo "wasm-opt failed"
        fi
    else
        echo "wasm-bindgen failed"
    fi
else
    echo "cargo build failed"
fi
