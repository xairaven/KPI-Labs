//
// Created by xairaven on 09.01.2022.
//

#ifndef MCW1_XAIMATRIX_H
#define MCW1_XAIMATRIX_H
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

bool legalMultiplicationOperation(int SIZE1, int SIZE2);

int getDet(int** matrix, int SIZE);

int** declareMatrix(int SIZE); //creating dynamic array
int** getQuadMinor(int** matrix, int h, int v, int matrixSize); //get matrix minor
int** matrixTranspose(int** matrix, int SIZE);

float multRowCol (int** matrix1, float** matrix2, int SIZE1, int SIZE2, int row, int col);

float** declareFloatMatrix(int SIZE);
float** inverseMatrix(int** matrix, int SIZE);
float** matrixFloatTranspose(float** matrix, int SIZE);
float** multMat (int** matrix1, float** matrix2, int SIZE1, int SIZE2);

void initialisingMatrix(int** matrix, const int* rawMatrix, int SIZE);//static array -> dynamic array
void printMatrix(int** matrix, int SIZE); //print matrix
void printFloatMatrix(float** matrix, int SIZE);
#endif //MCW1_XAIMATRIX_H