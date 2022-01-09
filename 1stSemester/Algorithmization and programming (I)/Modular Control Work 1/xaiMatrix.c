//
// Created by xairaven on 09.01.2022.
//
#include "xaiMatrix.h"

int getDet(int** matrix, int SIZE) {
    if (SIZE == 2) {
        return matrix[0][0]*matrix[1][1] - matrix[0][1]*matrix[1][0];
    }
    int total = 0;
    int num = SIZE;
    int* nums = calloc(num, sizeof(int));
    for (int i = 0; i < num; i++) {
        nums[i] = matrix[0][i] * getDet(getQuadMinor(matrix, 1, i + 1, SIZE - 1), SIZE - 1);
        if(i % 2 != 0) {
            nums[i] *= -1;
        }
    }
    for (int i = 0; i < num; i++) {
        total += nums[i];
    }
    return total;
}

int** declareMatrix(int SIZE) {
    int** matrix = (int **) malloc(SIZE * sizeof(int *));
    for(int i = 0; i < SIZE; i++) {
        matrix[i] = (int *) malloc(SIZE * sizeof(int));
    }
    return matrix;
}

int** getQuadMinor(int** matrix, int h, int v, int minorSize) {
    int** newData = declareMatrix(minorSize);
    for (int i = 0; i < minorSize; i++) {
        if (i < h - 1) {
            for (int j = 0; j < minorSize; j++) {
                if (j < v - 1) {
                    newData[i][j] = matrix[i][j];
                } else {
                    newData[i][j] = matrix[i][j + 1];
                }
            }
        } else {
            for (int j = 0; j < minorSize; j++) {
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

int** matrixTranspose(int** matrix, int SIZE) {
    int** transposeMatrix = declareMatrix(SIZE);
    for (int i = 0; i < SIZE; i++) {
        for (int j = 0; j < SIZE; j++) {
            transposeMatrix[j][i] = matrix[i][j];
        }
    }
    return transposeMatrix;
}

float** declareFloatMatrix(int SIZE) {
    float** matrix = (float **) malloc(SIZE * sizeof(float *));
    for(int i = 0; i < SIZE; i++) {
        matrix[i] = (float *) malloc(SIZE * sizeof(float));
    }
    return matrix;
}

float** inverseMatrix(int** matrix, int SIZE) {
    int det = getDet(matrix, SIZE);

    float** tempMatrix = declareFloatMatrix(SIZE);
    for (int i = 0; i < SIZE; i++) {
        for (int j = 0; j < SIZE; j++){
            int num = getDet(getQuadMinor(matrix, i+1, j+1, SIZE - 1),SIZE - 1);
            if ((i + j) % 2 != 0) {
                num *= -1;
            }
            tempMatrix[i][j] = (float) num / (float) det;
        }
    }
    tempMatrix = matrixFloatTranspose(tempMatrix,SIZE);
    return tempMatrix;
}

float** matrixFloatTranspose(float** matrix, int SIZE) {
    float** transposeMatrix = declareFloatMatrix(SIZE);
    for (int i = 0; i < SIZE; i++) {
        for (int j = 0; j < SIZE; j++) {
            transposeMatrix[j][i] = matrix[i][j];
        }
    }
    return transposeMatrix;
}

void initialisingMatrix(int** matrix, const int* rawMatrix, int SIZE) {
    int k = 0;
    for(int i = 0; i < SIZE; i++) {
        for (int j = 0; j < SIZE; j++, k++) {
            matrix[i][j] = rawMatrix[k];
        }
    }
}

void printMatrix(int** matrix, int SIZE) {
    for (int i = 0; i < SIZE; i++) {
        for (int j = 0; j < SIZE; j++) {
            printf("%d\t", matrix[i][j]);
        }
        printf("\n");
    }
}

void printFloatMatrix(float** matrix, int SIZE) {
    for (int i = 0; i < SIZE; i++) {
        for (int j = 0; j < SIZE; j++) {
            printf("%.3f\t", matrix[i][j]);
        }
        printf("\n");
    }
}

