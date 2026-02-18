#!/bin/bash

MAX_PROCESSES=4

MATRIX_DIMENSIONS=2000
SEED=345345

rm ./out/*
rm ./results/*

echo -e "----------   PARALLEL   ----------"
mpicc ./src/task.c -lm -o ./out/parallel
perf stat mpirun --allow-run-as-root -np $MAX_PROCESSES ./out/parallel $MATRIX_DIMENSIONS ./results/parallel.txt $SEED

echo -e "\n"

echo -e "---------- NON-PARALLEL ----------"
mpicc ./src/task.c -lm -o ./out/non-parallel
perf stat mpirun --allow-run-as-root -np 1 ./out/non-parallel $MATRIX_DIMENSIONS ./results/non-parallel.txt $SEED

chmod o+rw -R ./out/*
chmod o+rw -R ./results/*
