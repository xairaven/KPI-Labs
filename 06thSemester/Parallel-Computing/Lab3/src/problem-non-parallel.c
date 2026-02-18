#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <time.h>

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

const char* filePath = "./results/non-parallel.txt";

int iteration = 20;
const float dt = 0.1f;

int bodies;

Body *collection;

void bodyForce(Body *p, int length);
void randomizeBodies(int bodies);
void outputResults(FILE* file, Body *body, double time);

int main(int argc, char *argv[])
{
    if (argc != 2)
    {
        fprintf(stderr, "Usage: %s <number_of_bodies>\n", argv[0]);
        return 1;
    }

    bodies = atoi(argv[1]);

    clock_t start_time;

    start_time = clock();

	// Allocating memory for collection
    collection = malloc(sizeof(Body) * bodies);

    randomizeBodies(bodies);

    for (int iter = 0; iter < iteration; iter++)
    {
        bodyForce(collection, bodies);
    }

    double cpu_time_used = ((double) (clock() - start_time)) / CLOCKS_PER_SEC;

    // Write data to the file
    FILE *outputFile = fopen(filePath, "w");
    if (outputFile == NULL)
    {
        fprintf(stderr, "Error opening the file for writing.\n");
        return 1;
    }

    outputResults(outputFile, collection, cpu_time_used);
    fclose(outputFile);

    // Free memory
    free(collection);
    
    return 0;
}

void bodyForce(Body *p, int length)
{
    for (int i = 0; i < length; i++)
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

    for (int l = 0; l < length; l++)
    {
        p[l].x += p[l].vx * dt;
        p[l].y += p[l].vy * dt;
        p[l].z += p[l].vz * dt;
    }
}

void randomizeBodies(int bodies)
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
