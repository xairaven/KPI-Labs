#!/bin/bash

echo "-- Var 25 --"
# cat - concatenate files and print on the standard output
# /etc/passwd - file with user details
# whoami - prints user name
# grep -i: searching user in passwd (ignoring case)
cat /etc/passwd | grep -i $(whoami)