#include <stdio.h>
#include <stdbool.h>
void bubbleSort(int* a, int SIZE);
int maxElem(int *arr, int SIZE);
void textPrint(int* arr1, int SIZE1, int* arr2, int SIZE2);

int main(void) {
  printf("--- Task 1 ---\n");
  int a[] = {3, 4, 11, 6, 7};
  int b[] = {4, 2, 11, 1};
  const int SIZE_A = sizeof(a)/sizeof(a[0]);
  const int SIZE_B = sizeof(b)/sizeof(b[0]);
  textPrint(a, SIZE_A, b, SIZE_B);
  int maxElemA = maxElem(a, SIZE_A);
  int maxElemB = maxElem(b, SIZE_B);
  if (maxElemA > maxElemB) {
    printf("\n\nArray A has been sorted\nMax element in A = %d\n", maxElemA);
    bubbleSort(a, SIZE_A);
    textPrint(a, SIZE_A, b, SIZE_B);
  } else if (maxElemA < maxElemB) {
    printf("\n\nArray B has been sorted\nMax element in B = %d\n", maxElemB);
    bubbleSort(b, SIZE_B);
    textPrint(a, SIZE_A, b, SIZE_B);
  } else {
    printf("\n\nArrays A and B have been sorted\nMax element in A = %d\nMax element in B = %d\n", maxElemA, maxElemB);
    bubbleSort(a, SIZE_A);
    bubbleSort(b, SIZE_B);
    textPrint(a, SIZE_A, b, SIZE_B);
  }
  return 0;
}

void bubbleSort(int *arr, int SIZE) {
  bool sorted = false;
  int temp;
  while (!sorted) {
    sorted = true;
    for (int i = 0; i < SIZE - 1; i++) {
      if (*(arr + i) > *(arr + i + 1)) {
        temp = *(arr + i);
        *(arr + i) = *(arr + i + 1);
        *(arr + i + 1) = temp;
        sorted = false;
      }
    }
  }
}

int maxElem(int *arr, int SIZE) {
  int max = *arr;
  for (int i = 0; i < SIZE; i++) {
    if (max < *(arr + i)) {
      max = *(arr + i);
    }
  }
  return max;
}

void textPrint(int* arr1, int SIZE1, int* arr2, int SIZE2) {
  printf("Array 1:\n");
  for (int i = 0; i < SIZE1 ; i++) {
    printf("%d\t", arr1[i]);
  }
  printf("\n\nArray 2:\n");
  for (int i = 0; i < SIZE2 ; i++) {
    printf("%d\t", arr2[i]);
  }
}