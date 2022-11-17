#!/bin/bash

echo "-- Var 8 --"

FILE="Kovalyov.txt"
if [ -f $FILE ]; then       # if file exists
    echo "Access rights of file $FILE:"
    echo "$(ls -l $FILE)"   # ls command with key -l prints access rights
else
    echo "File has not found."
	exit 1
fi