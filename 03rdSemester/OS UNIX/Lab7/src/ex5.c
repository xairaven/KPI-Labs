#include <error.h>
#include <inttypes.h>
#include <pthread.h>
#include <semaphore.h>
#include <stdint.h>
#include <stdio.h>

void *WriteToFile(void *);
sem_t psem;
FILE *in; 

int main(int argc, char* argv[]) {
    pthread_t tidA, tidB;
    
    in = fopen("result.txt", "w");
    
    int number = 100;

    sem_init(&psem, 0, 0);
    sem_post(&psem);
    
    pthread_create(&tidA, NULL, &WriteToFile, (void *) &number);
    pthread_create(&tidB, NULL, &WriteToFile, (void *) &number);
    
    pthread_join(tidA, NULL);
    pthread_join(tidB, NULL);
    
    fclose(in);

    sem_destroy(&psem);

    return 0;
}

void *WriteToFile(void *f) {
    int max = *((int *) f);
    for (int i = 0; i <= max; i++) {
        sem_wait(&psem);
        fprintf(in, "-writetofilecounter i=%d \n", i);
        sem_post(&psem);
    }
    return NULL;
}
