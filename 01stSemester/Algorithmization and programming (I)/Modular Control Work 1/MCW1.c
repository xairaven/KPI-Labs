#include "xaiMatrix.h"

#define ORIG_SIZE 3 //dimensions of array. USER INPUT
int main(void) {
    int rawMatrix[ORIG_SIZE][ORIG_SIZE] = { //Matrix, quadratic. USER INPUT
            {2, 1, 2},
            {3, 2, 1},
            {2, 2, 2},
    };

    int SIZE = ORIG_SIZE;
    int** matrix = declareMatrix(SIZE); //declaring matrix
    initialisingMatrix(matrix, (int*) rawMatrix, SIZE); //initialising matrix

    printf("Matrix:\n");
    printMatrix(matrix, SIZE); //printing matrix

    int det = getDet(matrix, SIZE); //calculating determinant
    printf("\nDeterminant = %d\n", det);

    float** invMat = inverseMatrix(matrix,SIZE); //inversing matrix
    printf("\nInverse matrix:\n");
    printFloatMatrix(invMat, SIZE); //printing matrix

    float** identityMatrix = multMat(matrix, invMat, SIZE, SIZE);
    printf("\nIdentity matrix: \n");
    printFloatMatrix(identityMatrix, SIZE);
}