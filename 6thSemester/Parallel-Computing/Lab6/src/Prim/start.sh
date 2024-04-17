#!/bin/bash

MAX_PROCESSES=4
GENERATE_CSV=0

NODES=10000
EDGES=15000
MAX_WEIGHTS=100

INPUT_FILE="./graphs/input.csv"
OUTPUT_FOLDER="./results/"

# Removing files from folders
if [ -d "./results/" ] && [ "$(ls -A ./results/)"  ]; then
    rm -r ./results/*
fi
if [ -d "./out/" ] && [ "$(ls -A ./out/)"  ]; then
    rm -r ./out/*
fi


figlet -f starwars "Lab 6"


# Generating input (optional)
if [ "$GENERATE_CSV" -eq 1 ]; then
    echo -e "\nGenerating CSV file...\n"
    if [ -d "./graphs/" ] && [ "$(ls -A ./graphs/)"  ]; then
        rm -r ./graphs/*
    fi
    python3 src/generate_input_file.py $NODES $EDGES $MAX_WEIGHTS -o $INPUT_FILE
fi

# Compilation
mpicc ./src/prim.c -lm -o ./out/exec.out


# Running
echo -e "---------- NON-PARALLEL ----------"
perf stat mpirun --allow-run-as-root -np 1 ./out/exec.out $INPUT_FILE "$OUTPUT_FOLDER/non-parallel.txt"
echo -e "To check if the results are the same, the last 2 lines:"
tail -n 3 "$OUTPUT_FOLDER/non-parallel.txt"


echo -e "\n"

echo -e "----------   PARALLEL   ----------"
perf stat mpirun --allow-run-as-root -np $MAX_PROCESSES ./out/exec.out $INPUT_FILE "$OUTPUT_FOLDER/parallel.txt"
echo -e "To check if the results are the same, the last 2 lines:"
tail -n 3 "$OUTPUT_FOLDER/parallel.txt"

# Give permissions for others, if user want to delete folders
chmod o+rw -R ./results/
chmod o+rw -R ./out/
