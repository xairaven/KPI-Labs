#!/bin/bash

echo "-- Var 13 --"

FOLDER="var13-folder"

if [[ !(-d $FOLDER) ]]; then			# if dir doesn't exist
    mkdir $FOLDER						# mkdir - makedir
fi

echo "Label of the new directory: $FOLDER"
echo "$(tree $FOLDER)"                  # using $() construct for better-styled output
rmdir $FOLDER                           # if it's clear