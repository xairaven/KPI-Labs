#include <stdio.h>
#include <stdlib.h>
#include <math.h>
double formula (double a, double b, double x);

int main(int argc, char *argv[]) {
  double a, b, x, y;
  int choice = 0;
  printf("1. Args from main\n2. Args from scan\n3. Args in code\n");
  scanf("%d", &choice);
  switch (choice) {
    case 1:
      if (argc == 4) {
        y = formula (atof(argv[1]), atof(argv[2]), atof(argv[3]));
        if (y != 0) {
          printf("y = %lf\n", y);
        } else {
          printf("Не відповідає ОДЗ\n");
        }
      } else {
        printf ("Аргументів немає\n");
        break;
      }
      break;
    case 2:
      printf("Введіть числа\n");
      scanf("\n%lf%lf%lf", &a, &b, &x);
      y = formula (a, b, x);
      if (y != 0) {
        printf("y = %lf\n", y);
      } else {
        printf("Не відповідає ОДЗ\n");
      }
      break;
    case 3:
      a = 1; b = 2; x = 3; y = formula (a, b, x);
      if (y != 0) {
        printf("y = %lf\n", y);
      } else {
        printf("Не відповідає ОДЗ\n");
      }
      break;
    default:
      printf("Такого числа в списку немає\n");
      break;
  }
}

double formula (double a, double b, double x) {
  if (a != 0 && x != 0) {
    return log(a*x) + x + cos(x) + b*x + pow(a, b);
  } else {
    return 0;
  }
}
