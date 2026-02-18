#!/bin/bash

MAX_PROCESSES=4
NUMBERS_AMOUNT=1000000

INPUT_FILE="./data/input.txt"
OUTPUT_FILE="./data/output.txt"

rm ./out/*

if [ -f "./data/output.txt" ]; then
    rm $OUTPUT_FILE
fi

# Generating input file
python3 src/generate_random_numbers.py $NUMBERS_AMOUNT $INPUT_FILE
touch $OUTPUT_FILE

#-----------------------------------------------------------------------------------------------#

figlet -f starwars "Lab 5"

echo -e "---------- NON-PARALLEL ----------"
mpicc ./src/task.c -lm -o ./out/non-parallel
perf stat mpirun --allow-run-as-root -np 1 ./out/non-parallel $INPUT_FILE $OUTPUT_FILE
echo -e "Result: (last 100 bytes)\n"
tail -c 100 $OUTPUT_FILE


echo -e "\n"

echo -e "----------   PARALLEL   ----------"
mpicc ./src/task.c -lm -o ./out/parallel
perf stat mpirun --allow-run-as-root -np $MAX_PROCESSES ./out/parallel $INPUT_FILE $OUTPUT_FILE
echo -e "Result: (last 100 bytes)\n"
tail -c 100 $OUTPUT_FILE


chmod o+rw -R ./data/*
chmod o+rw -R ./out/*
