#include <stdio.h>
int countSize(char* string, int number);

int main(void) {
  char string[] = "Helloqwqw worldsss w qwe ww e";
  int number;
  printf("Start string:\t");
  for(int i = 0; string[i] != '\0'; i++) {
    printf("%c", string[i]);
  }
  printf("\nInput number of word:\t");
  scanf("%d", &number);
  int SIZE = countSize(string, number);
  if (SIZE > 0) {
    printf("\nSize of %dnd word = %d symbols", number, SIZE);
  } else {
    printf("\nLetter doesn't have %d words", number);
  }
  return 0;
}

int countSize(char* string, int number) {
  int count = 1;
  int SIZE = 0;
  for (int i = 1; string[i] != '\0'; i++) {
    if(string[i - 1] == ' ' && string[i] != ' ') {
      count++;
    }
    if (count == number && string[i] != ' ') {
      SIZE++;
    }
    if (number == 1 && i - 1 == 0 && string[i] != ' ' ) {
      SIZE++;
    }
  }
  return SIZE;
}
