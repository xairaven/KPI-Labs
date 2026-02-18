#!/bin/bash

echo "-- Var 17 --"

for FILE in ./*; do                     # for all files in pwd
    if [ -x $FILE ]; then               # if file executable
        echo "$FILE is executable."     # prints name
    fi
done