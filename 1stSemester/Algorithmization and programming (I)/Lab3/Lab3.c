#include <stdio.h>
#include <math.h>
void task1(void);
void task2(int x);
int factorial(int n);
int factorial_M(int n);

int main(void) {
  task1();
  printf("--- Task 2 ---\n\n");
  printf("Введіть x:\t");
  int x; scanf("%d", &x);
  task2(x);
  return 0;
}

void task1 (void) {
  printf("--- Task 1 ---\n\n");
  double start; double end; int count; double y;
  double a = 1; double b = 1;
  printf("Input interval.\n");
  printf("if it does not correspond to the range of acceptable values, the values ​​will be skipped\n\n");
  printf("Start:\t"); scanf("%lf", &start);
  printf("End:\t"); scanf("%lf", &end);
  printf("Amount of values:\t"); scanf("%d", &count);
  double step = fabs(end - start) / (double) count;
  while (start < end) {
    if (a*start > 0) {
      y = log(a*start) + start + cos(start) +b*start + pow(a, b);
      printf("x = %.4lf\t\ty = %.4lf", start, y);
    } else {
      printf("x = %.4lf\t\ty = NaN", start);
    }
    printf("\n");
    start += step;
  }
}

void task2 (int x) {
  double eps = 1000;
  double y = 1;
  for (int n = 1; fabs(y) < eps; n++) {
    y = (pow(-1, n + 1) * factorial_M(n) * pow(x, 2*n)) / (pow(2, n)* factorial(n+1));
    if (y > eps) {
      break;
    }
    printf("%lf\n", y);
  }
}

int factorial(int n) {
  int result = 1;
  for (int i = 2; i <= n; i++) {
    result *= i;
  }
  return result;
}

int factorial_M (int n) {
  int result = 1;
  for (int i = 1; i <= n; i+=2) {
    result *= i;
  }
  return result;
}