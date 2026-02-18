#!/bin/bash

echo "-- Var 21 --"

NAME="Archive.tar"              # custom name of archive
# tar - an archiving utility
# -cf - key for --create and --file (clarifying archive name)
# ./ - destination
tar -cf $NAME ./
echo "Done! Check $NAME"