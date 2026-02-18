#include <stdio.h>
#include "mpi.h"
int main(int argc, char* argv[]) {
    int ProcNum, ProcRank;
    
    MPI_Status Status;
    MPI_Init(&argc, &argv);
    
    MPI_Comm_rank(MPI_COMM_WORLD, &ProcRank);
    
    if (ProcRank == 0) { // Дії, виконувані тільки процесом з рангом 0
        printf ("Hello from process %d\n", ProcRank);
        MPI_Comm_size(MPI_COMM_WORLD, &ProcNum);
 
        for (int i = 1; i < ProcNum; i++) {
            MPI_Recv(&ProcRank, 1, MPI_INT, MPI_ANY_SOURCE, MPI_ANY_TAG, MPI_COMM_WORLD, &Status);
            printf("Hello from process %d\n", ProcRank);
        }
    } else // Повідомлення, що відправляється всіма процесами, крім процесу з рангом 0
        MPI_Send(&ProcRank, 1, MPI_INT, 0, 0, MPI_COMM_WORLD);
 
    
    MPI_Finalize();
 
    return 0;
}

