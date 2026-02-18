#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <stdbool.h>
double formula(double a, double c, double x);

int main(int argc, char* argv[]) {
  double a, c, x, y;
  int choice;
  bool command = true;
  printf("Data input:\n");
  printf("1. From console\n2. From command line\n3. Default\n");
  scanf("%d", &choice);
  switch(choice) {
    case 1:
      printf("Input 3 numbers (a, c, x)\n");
      scanf("%lf%lf%lf", &a, &c, &x);
      break;
    case 2:
      if (argc < 4) {
        command = false;
      } else {
        a = atof(argv[1]);
        c = atof(argv[2]);
        x = atof(argv[3]);
      }
      break;
    case 3:
      a = 1;
      c = 2;
      x = 3;
      break;
    default:
      printf("You chose wrong number\n");
      break;
  }
  if (((a > 0 && c > 0) || x <= 7) && command) {
    printf("Result = %.4lf", formula(a, c, x));
    } else {
      printf("Out of range or not enough arguments");
    }
  return 0;
}

double formula(double a, double c, double x) {
  if (x < 3) {
    return sin(a*c*x);
  } else if (x >= 3 && x <= 7) {
    return fabs(a*x + c);
  } else {
    return log(a*c*x);
  } 
}
