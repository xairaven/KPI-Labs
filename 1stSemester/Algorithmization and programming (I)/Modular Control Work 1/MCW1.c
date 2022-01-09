#include <stdio.h>
#include <stdlib.h>
#include <math.h>
int** declareMatrix(int SIZE);
void initialisingMatrix(int** matrix, int* rawMatrix, int SIZE);
void getMinor(int** matrix, int h, int v, int SIZE);
void printMatrix(int** matrix, int SIZE);

#define ORIG_SIZE 3
int main(void) {
    int rawMatrix[ORIG_SIZE][ORIG_SIZE] = {
            {2, 1, 2},
            {3, 2, 1},
            {2, 2, 2}
    };

    int SIZE = ORIG_SIZE;
    int** matrix = declareMatrix(SIZE);
    initialisingMatrix(matrix, (int*) rawMatrix, SIZE);
    printMatrix(matrix, SIZE);
}

void getMinor(int** matrix, int h, int v, int SIZE) {
    int** newData = declareMatrix(SIZE - 1);
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
