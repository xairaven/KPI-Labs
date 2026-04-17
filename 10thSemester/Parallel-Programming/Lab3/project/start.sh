#!/bin/bash
cargo build --release --bin Lab3

EXECUTABLE="./target/release/Lab3"
$EXECUTABLE -t 1 --log-level info