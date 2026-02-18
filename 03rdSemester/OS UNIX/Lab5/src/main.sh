#!/bin/bash

# Creating variable with name of the file
FILE="Kovalyov.txt"
{
	echo "Kovalyov Oleksandr Oleksiyovuch"
	echo "Academic group = TP-12"
	echo "Hobbies: Programming, learning Linux"
} | tee $FILE
# tee is a command in CLI using standard streams
# which reads standard input and writes it to both 
# standard output and one or more files, 
# effectively duplicating its input.