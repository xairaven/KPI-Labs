#include "xaiMatrix.h"
#include "xaiCryptoHill.h"

int* multCryptoLetterMat(int* matrix1, float** matrix2, int SIZE1, int SIZE2);

#define orig_size 2
int main() {
    int rawMatrix[orig_size][orig_size] = {
            {3, 2},
            {4, 2},
    };
    char* cypher = "belq";

    char alphabet[26] = {'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'};
    int* numChars = malloc(4 * sizeof(int));
    for (int i = 0; i < 4; i++) {
        for (int j = 0; j < 26; j++) {
            if(alphabet[j] == cypher[i]) {
                numChars[i] = j;
                break;
            }
        }
    }
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

    float** identityMatrix = multMat(matrix, invCrMatrix, SIZE, SIZE);
    printf("\nResult without mod26:\n");
    printFloatMatrix(identityMatrix, SIZE);

    float** identCryptoMatrix = multCryptoMat(matrix, invCrMatrix, SIZE, SIZE);
    printf("\nResult with mod26:\n");
    printFloatMatrix(identCryptoMatrix, SIZE);

    int tempMatrix[2];
    int* resultNumChars = malloc(4 * sizeof(int));
    for(int i = 0; i < 4; i += 2) {
        tempMatrix[i % 2] = numChars[i];
        tempMatrix[(i + 1) % 2] = numChars[i+1];
        int* temp2matrix = multCryptoLetterMat(tempMatrix, invCrMatrix, SIZE,SIZE);
        resultNumChars[i] = temp2matrix[0] % 26;
        resultNumChars[i + 1] = temp2matrix[1] % 26;
    }

    printf("\nresultNum:\n");
    for (int i = 0; i < 4; i++) {
        printf("%d\t", resultNumChars[i]);
    }
    printf("\nresult:\n");
    for (int i = 0; i < 4; i++) {
        printf("%c", alphabet[resultNumChars[i]]);
    }

    return 0;
}