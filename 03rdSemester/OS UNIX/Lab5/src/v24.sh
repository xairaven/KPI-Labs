#!/bin/bash

echo "-- Var 24 --"
# you can check net status by tool ifconfig from net-tools package
# is net-tools installed?
if [[ !($(dpkg -s net-tools | grep Status)) ]]; then
    # if not, install package by apt
    sudo apt install net-tools
fi
# using command
sudo ifconfig