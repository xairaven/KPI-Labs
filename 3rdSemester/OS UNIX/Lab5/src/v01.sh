#!/bin/bash

echo "-- Var 1 --" 		

FILENAME="Kovalyov"

if [ -f $FILENAME.txt ]; then 					# if file exists
	read -p "File exists! Input new name: " NEWFILENAME
	mv $FILENAME.txt $NEWFILENAME.txt			#renaming
	echo "Done! File renamed to $NEWFILE.txt"
else
	echo "File $FILENAME.txt doesn't exist"
fi