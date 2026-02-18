#!/bin/bash

echo "-- Var 10 --"

FILE="Kovalyov"
if [ -f $FILE.txt ]; then			# if file exists
	echo "Copying file.."
	cp $FILE.txt "$FILE-copy.txt"	# just copying file
	echo "Done! New name of the file is $FILE-copy.txt"
else 
	echo "Error: file has not found"
fi
