#!/bin/bash

rm ./out/*
rm ./reports/*


gcc ./src/task_parallel.c -o ./out/parallel -fopenmp
gcc ./src/task_nonp.c -o ./out/non-parallel


echo -e "---------------   PARALLEL   ---------------\n"
time ./out/parallel
perf stat ./out/parallel

echo -e "--------------- NON-PARALLEL ---------------\n"
time ./out/non-parallel
perf stat ./out/non-parallel

perf stat ./out/parallel >> ./reports/parallel.txt 2>&1
perf stat ./out/non-parallel >> ./reports/non-parallel.txt 2>&1
