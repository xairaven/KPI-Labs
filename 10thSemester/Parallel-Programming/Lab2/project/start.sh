#!/bin/bash
cargo build --release --bin Lab2

EXECUTABLE="./target/release/Lab2"
$EXECUTABLE -t 1 --log-level info