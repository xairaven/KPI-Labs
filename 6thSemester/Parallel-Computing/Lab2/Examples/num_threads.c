#include <stdio.h>
#include <omp.h>

int main(int argc, char *argv[]) {
    int num;

    #pragma omp parallel
    {
        if ((num=omp_get_thread_num())==0)
            printf("Всього потоків: %d\n", omp_get_num_threads());
        else
            printf("Потік номер %d\n",num);
    }
}
