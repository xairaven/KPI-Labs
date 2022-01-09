#include "xaiMatrix.h"

#define ORIG_SIZE 4 //dimensions of array. USER INPUT
int main(void) {
    int rawMatrix[ORIG_SIZE][ORIG_SIZE] = { //Matrix. USER INPUT
            {2, 4, 1, 1},
            {0, 2, 1, 0},
            {2, 1, 1, 3},
            {4, 0, 2, 3},
    };

    int SIZE = ORIG_SIZE;
    int** matrix = declareMatrix(SIZE);
    initialisingMatrix(matrix, (int*) rawMatrix, SIZE);

    int det = getDet(matrix, SIZE);
    printf("Determinant = %d\n", det);
}
