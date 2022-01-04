#include <stdio.h>
#include <stdlib.h>
#include <time.h>

void printMatrix(int* arr, int SIZE);
void algorythm_M(int* arr, int SIZE);

int main(void) {
  const int SIZE_ARR = 9;
  srand(time(NULL)); //Initialising random seed
  int arr[SIZE_ARR][SIZE_ARR]; 
  for (int i = 0; i < SIZE_ARR; i++) { //Randomising values
    for (int j = 0; j < SIZE_ARR; j++) {
      arr[i][j] = 1 + rand() % 99;
    }
  }
  printf("Start array:\n\n");
  printMatrix(*arr, SIZE_ARR);
  algorythm_M(*arr, SIZE_ARR);
  printf("\n\nModified array:\n\n");
  printMatrix(*arr, SIZE_ARR);

  return 0;
}

void printMatrix(int* arr, int SIZE) {
  for (int i = 0; i < SIZE; i++) {
    for (int j = 0; j < SIZE; j++) {
      printf("%d\t", *(arr + i * SIZE + j));
    }
    putchar('\n');
  }
}

void algorythm_M(int* arr, int SIZE) {
  int temp = 0;
  for (int i = 0; i < SIZE/2 + 1; i++) {
    for (int j = 0; j < SIZE; j++) {
      if ((i != j) && (i + j != 8 ) && (i != SIZE/2) ) {
        temp = *(arr + i * SIZE + j);
        *(arr + i * SIZE + j) = *(arr + (8 - i) * SIZE + j);
        *(arr + (8 - i) * SIZE + j) = temp;
      }
    }
  }
}

/* Algorythm without pointers
int temp = 0;
  for (int i = 0; i < SIZE_ARR/2 + 1; i++) {
    for (int j = 0; j < SIZE_ARR; j++) {
      if ((i != j) && (i + j != 8 ) && (i != SIZE_ARR/2) ) {
        temp = arr[i][j];
        arr[i][j] = arr[8 - i][j];
        arr[8 - i][j] = temp;
      }
    }
  }*/
