#!/bin/bash

echo "-- Var 14 --"

FILE="Kovalyov.txt"
if [ -f $FILE ]; then           # if file exists
    cat $FILE                   # cat - concatenate files and print on the standard output
else
    echo "File doesn't exist"
fi