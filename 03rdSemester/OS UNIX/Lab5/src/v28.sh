#!/bin/bash

echo "-- Var 28 --"

FILE="Kovalyov.txt"

# is file exist?
if [[ !(-f $FILE) ]]; then
	echo "File has not found."
	exit 1
fi

# command "read" reading input from standard input to var.
# -p key allows to print text input prompt
read -p "Input label of directory: " DIR

# creating directory
mkdir $DIR

# copying file to it
cp $FILE $DIR

echo "Done."