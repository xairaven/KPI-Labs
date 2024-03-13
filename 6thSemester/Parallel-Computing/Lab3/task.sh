#!/bin/bash

PARTICLES=10000

PROCESSES=2

rm ./out/*
rm ./results/*
rm ./reports/*

echo -e "----------   PARALLEL   ----------"
mpicc ./src/problem-parallel.c -lm -o ./out/parallel
perf stat -o ./reports/parallel.txt mpirun --allow-run-as-root -np $PROCESSES ./out/parallel $PARTICLES > /dev/null 2>&1
echo -e $(tail -n 1 ./results/parallel.txt)
echo -e "CPUs utilized (PROCESSES VARIABLE): $PROCESSES"
echo -e "CPUs utilized (REAL PERF): $(sed -n '6p' ./reports/parallel.txt | cut -d '#' -f 2 | cut -d " " -f 5)"

echo -e "\n"

echo -e "---------- NON-PARALLEL ----------"
gcc ./src/problem-non-parallel.c -lm -o ./out/non-parallel
perf stat -o ./reports/non-parallel.txt ./out/non-parallel $PARTICLES > /dev/null 2>&1
echo -e $(tail -n 1 ./results/non-parallel.txt)
echo -e "CPUs utilized (REAL PERF): $(sed -n '6p' ./reports/non-parallel.txt | cut -d '#' -f 2 | cut -d " " -f 5)"

chmod o+rw -R ./results/*
chmod o+rw -R ./reports/*
