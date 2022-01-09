#include "xaiMatrix.h"

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
    int** minor = getQuadMinor(matrix, 2, 1, &minorSIZE);
    printMatrix(minor, minorSIZE);
}
