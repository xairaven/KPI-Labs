#!/bin/bash

echo "-- Var 9 --"

FILE="Kovalyov.txt"
if [ -f $FILE ]; then 		# if file exists
	echo "Access rights of file $FILE before:"
	echo "$(ls -l $FILE)"	# ls command with key -l prints access rights
	chmod g-wx $FILE		# chmod changes access rights. group-(write|executable)
	echo "Access rights of file $FILE after:"
	echo "$(ls -l $FILE)"
else
	echo "Error! File $FILE has not found"
fi
