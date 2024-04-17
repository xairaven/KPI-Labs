#!/bin/bash

MAX_PROCESSES=4
GENERATE_INPUT=0

DIM=30

INPUT_FILE="./graphs/input${DIM}x${DIM}.txt"
OUTPUT_FOLDER="./results"

# Removing files from folders
if [ -d "./results/" ] && [ "$(ls -A ./results/)"  ]; then
    rm -r ./results/*
fi
if [ -d "./out/" ] && [ "$(ls -A ./out/)"  ]; then
    rm -r ./out/*
fi


figlet -f starwars "Lab 6"


# Generating input (optional)
if [ -f $INPUT_FILE ]; then
	echo -e "\nUsing file $INPUT_FILE...\n"
else
	python3 ./src/generate_input.py $DIM $INPUT_FILE
fi

# Compilation
mpicc ./src/floyd.c -lm -o ./out/exec.out


# Running
echo -e "---------- NON-PARALLEL ----------"
mpirun -np 1 ./out/exec.out "$OUTPUT_FOLDER/non-parallel.txt" < $INPUT_FILE
echo -e "To check if the results are the same, the last 100 bytes:"
tail -c 100 "$OUTPUT_FOLDER/non-parallel.txt"


echo -e "\n"

echo -e "----------   PARALLEL   ----------"
mpirun -np $MAX_PROCESSES ./out/exec.out "$OUTPUT_FOLDER/parallel.txt" < $INPUT_FILE
echo -e "To check if the results are the same, the last 100 bytes:"
tail -c 100 "$OUTPUT_FOLDER/parallel.txt"

# Give permissions for others, if user want to delete folders
chmod o+rw -R ./graphs/
chmod o+rw -R ./results/
chmod o+rw -R ./out/
