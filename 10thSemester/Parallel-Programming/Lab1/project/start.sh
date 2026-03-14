#!/bin/bash
cargo build --release --bin Lab1

EXECUTABLE="./target/release/Lab1"
$EXECUTABLE -t 1 --log-level info