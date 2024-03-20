#include <math.h>
#include <mpi.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/types.h>
#include <sys/times.h>
#include <sys/time.h>
#include <time.h>
#include <unistd.h>

// MPI Variables
#define MASTER 0
int total_processes;
int rank_process;

// File Variables
char *output_filename;

// Matrix Variables
#define MAXN 5000 
int N;
double A[MAXN][MAXN], B[MAXN], X[MAXN];

// Prototypes
void gauss();
void back_substitution();
void finalize();

// Function for random based on time
unsigned int time_seed()
{
    struct timeval t;
    struct timezone tzdummy;

    gettimeofday(&t, &tzdummy);
    return (unsigned int)(t.tv_usec);
}

// Command line program parameters
void parameters(int argc, char **argv)
{
	// There must be atleast one argument -- matrix dimension
	if (argc < 2) {
		if (rank_process == MASTER)
        {
            printf("Use: %s <matrix_dimension> [output_filename] [seed]\n", argv[0]);
        }
        
		finalize();
	}

	
	// First argument - matrix dimension
	N = atoi(argv[1]);
	if (N < 1 || N > MAXN) {
        if (rank_process == MASTER) {
            printf("N = %i is out of range.\n", N);
        }
        
		finalize();
    }
	// Printing matrix dimensions
    if (rank_process == MASTER)
    {
        printf("- Matrix Dimension N = %i.\n", N);
    }
        
		
	// Second argument - output filename 
	if (argc >= 3) {
		if (rank_process == MASTER) {
			int length = strlen(argv[2]);
            
			output_filename = (char *) malloc(length + 1);
			output_filename = argv[2];
            
			printf("- Output filename: %s\n", output_filename);
		}
	} else {
		output_filename = "output.txt";
		if (rank_process == MASTER) {
			printf("- Output filename: %s\n", output_filename);
		}
	}
	
	// Third argument - Seed
	if (argc == 4) {
		int seed = atoi(argv[3]);
        srand(seed);

        if (rank_process == MASTER)
        {
            printf("- Seed = %i\n", seed);
        }
	} else {
		int seed = time_seed();
		srand(seed);
		if (rank_process == MASTER) {
            printf("- Random seed = %i\n", seed);
		}
	}
}

// Initialize A, B and X
void initializeInput()
{
    int row, col;

    printf("\nInitializing input...\n");

    for (col = 0; col < N; col++)
    {
        for (row = 0; row < N; row++)
        {
            A[row][col] = (double)rand() / 32768.0;
        }
        B[col] = (double)rand() / 32768.0;
        X[col] = 0.0;
    }
}

// Write input matrices
void write_entries(FILE *filePtr)
{
    int row, col;

    fprintf(filePtr, "A =\n\t");
    for (row = 0; row < N; row++)
    {
        for (col = 0; col < N; col++)
        {
            fprintf(filePtr, "%9.1lf%s", A[row][col], (col < N - 1) ? ", " : ";\n\t");
        }
    }
    fprintf(filePtr, "\nB = [");
    for (col = 0; col < N; col++)
    {
        fprintf(filePtr, "%9.1lf%s", B[col], (col < N - 1) ? "; " : "]\n");
    }
}

void print_X(FILE *filePtr)
{
    int row;

    fprintf(filePtr, "\nX = [");
    for (row = 0; row < N; row++)
    {
        fprintf(filePtr, "%9.1lf%s", X[row], (row < N - 1) ? "; " : "]\n");
    }
	
	fprintf(filePtr, "\n");
}

int main(int argc, char **argv)
{
    // Variables to calculate time
    struct timeval etstart, etstop;
    struct timezone tzdummy;
    clock_t etstart2, etstop2;
    unsigned long long usecstart, usecstop;
    struct tms cputstart, cputstop;
    FILE *filePtr;

    // Initialize MPI
    MPI_Init(&argc, &argv);

    // Number of processes
    MPI_Comm_size(MPI_COMM_WORLD, &total_processes);

    // Process rank
    MPI_Comm_rank(MPI_COMM_WORLD, &rank_process);

    parameters(argc, argv);

    // File to write the results
    if (rank_process == MASTER)
    {
        filePtr = fopen(output_filename, "w+");
        if (filePtr == NULL)
        {
            printf("There was an error while file opening. Exiting program...");
			
			finalize();
        }

        initializeInput();
		
        printf("\nStart.\n");
        gettimeofday(&etstart, &tzdummy);
        etstart2 = times(&cputstart);
    }

    // Gaussian elimination 
    gauss();

    if (rank_process == MASTER)
    {

        write_entries(filePtr);

        gettimeofday(&etstop, &tzdummy);
        etstop2 = times(&cputstop);
        printf("Stop.\n");
        usecstart = (unsigned long long)etstart.tv_sec * 1000000 + etstart.tv_usec;
        usecstop = (unsigned long long)etstop.tv_sec * 1000000 + etstop.tv_usec;

        // Show result
        print_X(filePtr);

        // Show time results 
		printf("\nDone!\n");
        printf("Time spent: %g ms.\n", (float)(usecstop - usecstart) / (float)1000);
    }

    finalize();
}

void gauss()
{

    int norm, row, col, multiplier[N], rownum[N];

    // Process master (0) broadcast data for all processes
    MPI_Bcast(&A[0][0], MAXN * MAXN, MPI_DOUBLE, 0, MPI_COMM_WORLD);
    MPI_Bcast(B, N, MPI_DOUBLE, 0, MPI_COMM_WORLD);

    for (row = 0; row < N; row++)
    {
        rownum[row] = row % total_processes;
    }

    for (norm = 0; norm < N; norm++)
    {
        // Processor 0 transmits the line of the outer loop being processed for all other processors
        MPI_Bcast(&A[norm][norm], N - norm, MPI_DOUBLE, rownum[norm], MPI_COMM_WORLD);
        MPI_Bcast(&B[norm], 1, MPI_DOUBLE, rownum[norm], MPI_COMM_WORLD);
        
		for (row = norm + 1; row < N; row++)
        {
            if (rownum[row] == rank_process)
            {
                multiplier[row] = A[row][norm] / A[norm][norm];
            }
        }
        for (row = norm + 1; row < N; row++)
        {
            if (rownum[row] == rank_process)
            {
                for (col = 0; col < N; col++)
                {
                    A[row][col] = A[row][col] - (multiplier[row] * A[norm][col]);
                }
                B[row] = B[row] - (multiplier[row] * B[norm]);
            }
        }
    }

    back_substitution();
}

void back_substitution()
{
    int row, col;
    if (rank_process == MASTER)
    {
        for (row = N - 1; row >= 0; row--)
        {
            X[row] = B[row];
            for (col = N - 1; col > row; col--)
            {
                X[row] -= A[row][col] * X[col];
            }
            X[row] /= A[row][row];
        }
    }
}

void finalize() {
    MPI_Finalize();

    exit(0);
}