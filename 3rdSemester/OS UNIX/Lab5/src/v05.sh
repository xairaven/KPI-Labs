#!/bin/bash

echo "-- Var 5 --"

FILE="Kovalyov.txt"

if [[ !(-f $FILE) ]]; then			# if file doesn't exist
	echo "File has not found."
	exit 1
fi

# command "read" reading input from standard input to var.
# -p key allows to print text input prompt
# grep is used here for finding a string
# -i key ignores cases of letters
read -p "Input string that you want to find: " STRING
LINE=$(grep -i $STRING $FILE)

# $? - code result of previous operation
if [ $? -eq 0 ]; then
	echo "String found! Result: "
	echo "$LINE"
else
	echo "String wasn't found."
fi