#include "mpi.h"
#include <stdio.h>

int main(int argc, char *argv[]) {
    int ProcNum, ProcRank;
    
    // <програмний код без використання MPI-функцій>
    
    MPI_Init ( &argc, &argv );
    MPI_Comm_size ( MPI_COMM_WORLD, &ProcNum);
    MPI_Comm_rank ( MPI_COMM_WORLD, &ProcRank);
    
    //<програмний код з використанням MPI-функцій>
 
    MPI_Finalize();

    printf("Comm (proc) size: %d\n", ProcNum);
    printf("Comm (proc) rank: %d\n", ProcRank);
 
    // <програмний код без використання MPI-функцій>
    
    return 0;
}
