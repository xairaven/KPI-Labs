//
// Created by xairaven on 09.01.2022.
//

#ifndef MCW1_XAIMATRIX_H
#define MCW1_XAIMATRIX_H
#include <stdio.h>
#include <stdlib.h>
int** declareMatrix(int SIZE); //creating dynamic array
void initialisingMatrix(int** matrix, const int* rawMatrix, int SIZE);//static array -> dynamic array
int** getQuadMinor(int** matrix, int h, int v, int* minorSIZE);//get matrix minor
void printMatrix(int** matrix, int SIZE); //print matrix
#endif //MCW1_XAIMATRIX_H
