#include <stdio.h>

#define N 50

int main(int argc, char *argv[])
{
    double a[N], b[N], c[N];
    int i;
 
    // ініціалізація масивів
    for (i = 0; i < N; i++) {
        a[i] = i * 1.0;
        b[i] = i * 2.0;
    }
 
    // обчислюємо суму відповідних елементів масивів
    for (i = 0; i < N; i++)
        c[i] = a[i] + b[i];
    
    printf ("%f\n", c[5]);
    
    return 0;
}
