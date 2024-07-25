#!/bin/bash
cargo build --release
exec "../target/release/rust_hyper" $@
