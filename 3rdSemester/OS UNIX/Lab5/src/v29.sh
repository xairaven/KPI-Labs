#!/bin/bash

echo "-- Var 29 --"

FILE="Kovalyov.txt"

# is file exist?
if [[ !(-f $FILE) ]]; then
	echo "File has not found."
	exit 1
fi

# if (amount-of-args) != 2
if [ $# -ne 2 ]; then
    echo "There have to be 2 args."
    exit 1
fi

echo "First number: $1"
echo "Second number: $2"

# construction $((smth)) evaluates expression
SUM=$(($1 + $2))

echo "SUM: $SUM" | tee -a $FILE