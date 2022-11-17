#!/bin/bash

echo "-- Var 11 --"

FOLDER="var11-folder"
if [ -d $FOLDER ]; then             # if folder already exists
    echo "Folder $FOLDER already exists!"
else
    echo "Creating folder \"$FOLDER\":"
    mkdir $FOLDER                   # if not, creating
fi

if [ -d $FOLDER-copy ]; then        # if copy folder exists..
    echo "Copy of $FOLDER already exists."
else
    echo "Copying..."
    cp -r $FOLDER "$FOLDER-copy"    # copy main folder recursively
fi

echo "List of files:"
# finding all folders in this dir with name of main folder
echo "$(find . -type d -name "$FOLDER*")"  