#include "xaiMatrix.h"
#include "xaiCryptoHill.h"

#define orig_size 2
int main() {
    int rawMatrix[orig_size][orig_size] = {
            {1, 4},
            {3, 9},
    };

    const int SIZE = orig_size;
    int** matrix = declareMatrix(SIZE);
    initialisingMatrix(matrix, (int*) rawMatrix, SIZE);
    printMatrix(matrix, SIZE);
    int det = getDet(matrix, SIZE);
    printf("\nDeterminant = %d\n", det);

    float** invMatrix = inverseMatrix(matrix, SIZE);
    printf("\nInverse matrix:\n");
    printFloatMatrix(invMatrix, SIZE);

    float** invCrMatrix = inverseCryptoMatrix(matrix, SIZE);
    printf("\nInverse matrix:\n");
    printFloatMatrix(invCrMatrix, SIZE);

    char alphabet[26] = {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'};
    return 0;
}
