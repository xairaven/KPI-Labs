#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <mpi.h>

#define SOFTENING 1e-9f

typedef struct
{
    float x;
    float y;
    float z;
    float vx;
    float vy;
    float vz;
} Body;

const char* filePath = "./results/parallel.txt";

int rank;              
int process;
MPI_Datatype MPIbody; 

int iteration = 20;
const float dt = 0.1f;

int bodies;

Body *collection;     

void bodyForce(Body *p, int start, int length);    
void randomizeBodies(Body *collection, int bodies);
void outputResults(FILE* file, Body *body, double time);                   

int main(int argc, char *argv[])
{
    MPI_Init(&argc, &argv);                      
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);        
    MPI_Comm_size(MPI_COMM_WORLD, &process);     
    MPI_Type_contiguous(6, MPI_FLOAT, &MPIbody); 
    MPI_Type_commit(&MPIbody);                   

    bodies = atoi(argv[1]); 

	// Allocating memory for collection
    collection = malloc(sizeof(Body) * bodies); 
    int part = bodies / process;               
    int rest = bodies % process;               
    int sum = 0;                                
    int *dspl, *sc;                            
    dspl = malloc(sizeof(int) * process);       
    sc = malloc(sizeof(int) * process);        
    double times;                               

    times = MPI_Wtime(); 

    for (int i = 0; i < process; i++) 
    {
        sc[i] = part; 
        if (rest > 0) 
        {
            sc[i]++;
            rest--;
        }
        dspl[i] = sum; 
        sum += sc[i];  
    }

    randomizeBodies(collection, bodies); 

    for (int i = 0; i < iteration; i++) {
        MPI_Bcast(collection, bodies, MPIbody, 0, MPI_COMM_WORLD);

        bodyForce(collection, dspl[rank], sc[rank]);

        MPI_Gatherv(&collection[dspl[rank]], sc[rank], MPIbody, collection, sc, dspl, MPIbody, 0, MPI_COMM_WORLD);
    }

    if (rank == 0) 
    {
        double timee = MPI_Wtime();
        double end = timee - times;
        // Write data to the file
     	FILE *outputFile = fopen(filePath, "w");
    	if (outputFile == NULL)
    	{
        	fprintf(stderr, "Error opening the file for writing.\n");
        	return 1;
    	}

    	outputResults(outputFile, collection, timee);

    	fclose(outputFile);
    }

    MPI_Type_free(&MPIbody);
    free(collection);       
    free(sc);               
    free(dspl);
    MPI_Finalize(); 
    return 0;
}

void bodyForce(Body *p, int start, int length)
{
    for (int i = start; i < start + length; i++)
    {
        float Fx = 0.0f;
        float Fy = 0.0f;
        float Fz = 0.0f;

        for (int j = 0; j < bodies; j++)
        {
            float dx = p[j].x - p[i].x;
            float dy = p[j].y - p[i].y;
            float dz = p[j].z - p[i].z;
            float distSqr = dx * dx + dy * dy + dz * dz + SOFTENING;
            float invDist = 1.0f / sqrtf(distSqr);
            float invDist3 = invDist * invDist * invDist;

            Fx += dx * invDist3;
            Fy += dy * invDist3;
            Fz += dz * invDist3;
        }
        p[i].vx += dt * Fx;
        p[i].vy += dt * Fy;
        p[i].vz += dt * Fz;
    }

    for (int l = start; l < start + length; l++)
    {
        p[l].x += p[l].vx * dt;
        p[l].y += p[l].vy * dt;
        p[l].z += p[l].vz * dt;
    }
}

void randomizeBodies(Body *collection, int bodies)
{
    for (int i = 0; i < bodies; i++)
    {
        collection[i].x = 2.0f * (rand() / (float)RAND_MAX) - 1.0f;
        collection[i].y = 2.0f * (rand() / (float)RAND_MAX) - 1.0f;
        collection[i].z = 2.0f * (rand() / (float)RAND_MAX) - 1.0f;
        collection[i].vx = 2.0f * (rand() / (float)RAND_MAX) - 1.0f;
        collection[i].vy = 2.0f * (rand() / (float)RAND_MAX) - 1.0f;
        collection[i].vz = 2.0f * (rand() / (float)RAND_MAX) - 1.0f;
    }
}

void outputResults(FILE *file, Body *body, double time)
{
    for (int i = 0; i < bodies; i++)
    {
        fprintf(file, "Body %d\n", i);
        fprintf(file, "x  = %f\ny  = %f\nz  = %f\nvx = %f\nvy = %f\nvz = %f\n\n", body[i].x, body[i].y, body[i].z, body[i].vx, body[i].vy, body[i].vz);
        fflush(file);
    }

    fprintf(file, "Time: %.6f seconds", time);
    fflush(file);
}
