#!/bin/bash

echo "-- Var 30 --"

FILE="Kovalyov.txt"

# is file exist?
if [[ !(-f $FILE) ]]; then
	echo "File has not found."
	exit 1
fi

# command "read" reading input from standard input to var.
# -p key allows to print text input prompt
read -p "Input your birthday, please: " BIRTHDAY

# >> - appending pipe
echo "Birthday: $BIRTHDAY" >> $FILE
echo "Done!"