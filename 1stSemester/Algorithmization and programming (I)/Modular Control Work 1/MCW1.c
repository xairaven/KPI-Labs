#include <stdio.h>
#include <stdlib.h>
#include <math.h>
int** declareMatrix(int SIZE); //creating dynamic array
void initialisingMatrix(int** matrix, int* rawMatrix, int SIZE);//static array -> dynamic array
int** getMinor(int** matrix, int h, int v, int* minorSIZE);//get matrix minor
void printMatrix(int** matrix, int SIZE); //print matrix

#define ORIG_SIZE 3 //dimensions of array. USER INPUT
int main(void) {
    int rawMatrix[ORIG_SIZE][ORIG_SIZE] = { //Matrix. USER INPUT
            {2, 1, 2},
            {3, 2, 1},
            {2, 2, 2}
    };

    int SIZE = ORIG_SIZE;
    int minorSIZE = SIZE;
    int** matrix = declareMatrix(SIZE);
    initialisingMatrix(matrix, (int*) rawMatrix, SIZE);

    int** minor = getMinor(matrix, 2, 1, &minorSIZE);
    printMatrix(minor, minorSIZE);
}

int** getMinor(int** matrix, int h, int v, int* minorSIZE) {
    *minorSIZE -= 1;
    int** newData = declareMatrix((*minorSIZE) - 1);
    for (int i = 0; i < *minorSIZE; i++) {
        if (i < h - 1) {
            for (int j = 0; j < *minorSIZE; j++) {
                if (j < v - 1) {
                    newData[i][j] = matrix[i][j];
                } else {
                    newData[i][j] = matrix[i][j + 1];
                }
            }
        } else {
            for (int j = 0; j < *minorSIZE; j++) {
                if (j < v - 1) {
                    newData[i][j] = matrix[i + 1][j];
                } else {
                    newData[i][j] = matrix[i + 1][j + 1];
                }
            }
        }
    }
    return newData;
}

void printMatrix(int** matrix, int SIZE) {
    for (int i = 0; i < SIZE; i++) {
        for (int j = 0; j < SIZE; j++) {
            printf("%d\t", matrix[i][j]);
        }
        printf("\n");
    }
}

int** declareMatrix(int SIZE) {
    int** matrix = (int **) malloc(SIZE * sizeof(int *));
    for(int i = 0; i < SIZE; i++) {
        matrix[i] = (int *) malloc(SIZE * sizeof(int));
    }
    return matrix;
}

void initialisingMatrix(int** matrix, int* rawMatrix, int SIZE) {
    int k = 0;
    for(int i = 0; i < SIZE; i++) {
        for (int j = 0; j < SIZE; j++, k++) {
            matrix[i][j] = rawMatrix[k];
        }
    }
}
