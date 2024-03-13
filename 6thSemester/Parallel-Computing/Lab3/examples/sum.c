#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include "mpi.h"

int main(int argc, char* argv[]) {
    srand(time(NULL));
    double x[100], TotalSum, ProcSum = 0.0;
    
    int ProcRank, ProcNum, N=100;
    MPI_Status Status;

    // ініціалізація
    MPI_Init(&argc,&argv);
    MPI_Comm_size(MPI_COMM_WORLD,&ProcNum);
    MPI_Comm_rank(MPI_COMM_WORLD,&ProcRank);

    // підготовка даних (заповнення масиву x випадковими числами від 0 до 1)
    if (ProcRank == 0)
        for (int i = 0; i < N; i++)
            x[i]=(double)rand()/RAND_MAX;
    
    // розсилання даних усім процесам
    MPI_Bcast(x, N, MPI_DOUBLE, 0, MPI_COMM_WORLD);
            
    // обчислення часткової суми вектора кожним процесом
    // кожний процес підсумовує елементи вектора x з індексами від i1 до i2
            
    int k = N / ProcNum;
    int i1 = k * ProcRank;
    int i2 = k * (ProcRank + 1);

    if (ProcRank == ProcNum-1)
        i2 = N;

    for ( int i = i1; i < i2; i++ )
        ProcSum += x[i];
            
    // збирання часткових сум процесом з рангом 0
    if (ProcRank == 0) {
        TotalSum = ProcSum;
        for ( int i=1; i < ProcNum; i++ ) {
            MPI_Recv(&ProcSum, 1, MPI_DOUBLE, MPI_ANY_SOURCE, 0, MPI_COMM_WORLD, &Status);
            TotalSum += ProcSum;
        }
    } else // решта процесів відсилає свої часткові суми
        MPI_Send(&ProcSum, 1, MPI_DOUBLE, 0, 0, MPI_COMM_WORLD);
                
    // виведення результату
    if ( ProcRank == 0 )
        printf("Total Sum = %f\n",TotalSum);
    
    MPI_Finalize();
    return 0;
}
