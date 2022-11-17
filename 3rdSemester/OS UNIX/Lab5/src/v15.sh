#!/bin/bash

echo "-- Var 15 --"

FILE="Kovalyov"
GROUP="TP-12"
if [[ !(-f "$FILE.txt") ]]; then    # if file doesn't exist
    echo "File doesn't exist"
    exit 1
fi

cp "$FILE.txt" "$FILE-copied.txt"   # copying file

{
    sed 's/TP-12//' "$FILE.txt"     # s - replacing, TP-12 by ""
} > "$FILE-copied.txt"              # and input final text to copied file
echo "Done. Result: $FILE-copied.txt"