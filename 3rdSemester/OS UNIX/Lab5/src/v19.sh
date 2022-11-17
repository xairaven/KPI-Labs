#!/bin/bash

echo "-- Var 19 --"

FILE="Kovalyov.txt"
# stat - display file or file system status
# --printf - printing custom-styled text
# %w - "birth" of file
stat --printf "File $FILE was created %w\n" $FILE