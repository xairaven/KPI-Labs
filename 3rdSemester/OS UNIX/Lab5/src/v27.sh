#!/bin/bash

echo "-- Var 27 --"

FILE="Kovalyov"

# is file exist?
if [[ !(-f "$FILE.txt") ]]; then
	echo "File has not found."
	exit 1
fi

# command "read" reading input from standard input to var.
# -p key allows to print text input prompt
read -p "Input label of copy document: " NEWFILE

# creating new file
touch "$NEWFILE.txt"

# copying text using pipe
cat "$FILE.txt" > "$NEWFILE.txt"

echo "Done. Wrote into $NEWFILE.txt"