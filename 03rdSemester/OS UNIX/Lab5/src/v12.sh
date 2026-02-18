#!/bin/bash

echo "-- Var 12 --"

FILE="Kovalyov.txt"
if [[ !(-f $FILE) ]]; then				# if file doesn't exist
    echo "Error: file has not found"
	exit 1
fi

FOLDER="var12-folder"
if [[ !(-d $FOLDER) ]]; then			# if dir doesn't exist
    mkdir $FOLDER						# mkdir - makedir
fi

echo "Moving file to folder \"$FOLDER\".."	
mv $FILE $FOLDER						# move file to folder
echo "Done!"

echo "$(tree $FOLDER)"					# using $() construct for better-styled output