//
// Created by xairaven on 10.01.2022.
//

#include "xaiCryptoHill.h"

float** inverseCryptoMatrix(int** matrix, int SIZE) {
    int det = getDet(matrix, SIZE);
    int detCopy = det;
    float** tempMatrix = declareFloatMatrix(SIZE);
    for (int i = 0; i < SIZE; i++) {
        for (int j = 0; j < SIZE; j++){
            detCopy = det;
            int num = getDet(getQuadMinor(matrix, i+1, j+1, SIZE - 1),SIZE - 1);
            if ((i + j) % 2 != 0) {
                num *= -1;
            }
            if (det < 0) {
                num *= -1;
                detCopy *= -1;
            }
            tempMatrix[i][j] = (float) num / (float) detCopy;
            for (int k = 1; fmod(tempMatrix[i][j], 1) != 0; k++) {
                tempMatrix[i][j] = (float) (num +  26 * k)  / (float) detCopy;
            }
            for(int k = 1; tempMatrix[i][j] < 0; k++) {
                tempMatrix[i][j] =  tempMatrix[i][j] + (float) (26 * k);
            }
        }
    }
    tempMatrix = matrixFloatTranspose(tempMatrix,SIZE);
    return tempMatrix;
}

float** multCryptoMat(int** matrix1, float** matrix2, int SIZE1, int SIZE2) {
    if (legalMultiplicationOperation(SIZE1, SIZE2)) {
        float** result = declareFloatMatrix(SIZE1);
        for (int i = 0; i < SIZE1; i++) {
            for (int j = 0; j < SIZE2; j++) {
                result[i][j] = multRowCol(matrix1, matrix2, SIZE1,SIZE2, i, j);
                result[i][j] = fmod(result[i][j], 26);
            }
        }
        return result;
    } else {
        printf("\nMultiplication operation is illegal\n");
        return NULL;
    }
}

int* multCryptoLetterMat(int* matrix1, float** matrix2, int SIZE1, int SIZE2) {
    int* result = malloc(SIZE1 * sizeof(int));
    result[0] = 0; result[1] = 0;
    for (int i = 0; i < SIZE1; i++) {
        for (int j = 0; j < SIZE2; j++) {
            result[i] += (int) matrix2[i][j] * matrix1[j];
        }
    }
    return result;
}