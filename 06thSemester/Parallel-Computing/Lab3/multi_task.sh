#!/bin/bash

mpirun --hostfile ./hostfile -n 4 /cloud/parallel 10000

echo -e "---------- HOSTS ----------"
head -n 3 /etc/hosts

echo -e "\n--------- HOSTFILE ---------"
cat ./hostfile

echo -e "\n(Last Result) $(tail -n 1 ./results/parallel.txt)"
