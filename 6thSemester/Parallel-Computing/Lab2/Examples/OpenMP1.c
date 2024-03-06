#include <stdio.h>
#include <omp.h>

#define N 50

int main(int argc, char *argv[])
{
    double a[N], b[N], c[N];
    int i;
    omp_set_dynamic(0); // заборонити бібліотеці OpenMP змінювати кількість потоків під час виконання програми
    omp_set_num_threads(5); // встановити загальну кількість потоків рівною 5
 
    // ініціалізація масивів
    for (i = 0; i < N; i++) {
        a[i] = i * 1.0;
        b[i] = i * 2.0;
    }
 
    // обчислюємо суму відповідних елементів масивів
    #pragma omp parallel for shared(a, b, c) private(i)
    for (i = 0; i < N; i++)
        c[i] = a[i] + b[i];
    
    printf ("%f\n", c[5]);
    
    return 0;
}
