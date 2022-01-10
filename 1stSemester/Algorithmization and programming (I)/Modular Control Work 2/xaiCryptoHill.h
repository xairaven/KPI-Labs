//
// Created by xairaven on 10.01.2022.
//

#ifndef MCW2_XAICRYPTOHILL_H
#define MCW2_XAICRYPTOHILL_H
#include "xaiMatrix.h"

float** inverseCryptoMatrix(int** matrix, int SIZE);
float** multCryptoMat(int** matrix1, float** matrix2, int SIZE1, int SIZE2);
int* multCryptoLetterMat(int* matrix1, float** matrix2, int SIZE1, int SIZE2);

#endif //MCW2_XAICRYPTOHILL_H
