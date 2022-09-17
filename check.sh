#!/bin/bash

rustup target add x86_64-unknown-linux-gnu

cargo check --target x86_64-unknown-linux-gnu
cargo clippy --target x86_64-unknown-linux-gnu
