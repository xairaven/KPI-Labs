#!/bin/bash

# Perf tool
apt install linux-tools-common linux-tools-generic linux-tools-`uname -r` -y

# Compiling
apt install gcc -y
apt install mpich -y
