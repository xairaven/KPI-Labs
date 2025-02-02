#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>
#define NUM_THREADS 6

void *thread_function(void *arg);

int main() {
    int res;
    int lots_of_threads;
    pthread_t a_thread[NUM_THREADS];
    void *thread_result;
    srand ((unsigned) time (NULL));

    for (lots_of_threads = 0; lots_of_threads < NUM_THREADS; lots_of_threads++) {
        res = pthread_create(&(a_thread[lots_of_threads]), NULL,
                             thread_function, (void *) &lots_of_threads);
        if (res != 0) {
            perror("Thread creation failed");

	        exit(EXIT_FAILURE);
        }
    }

    printf("Waiting for threads to finish...\n");
    
    for (lots_of_threads = NUM_THREADS - 1; lots_of_threads >= 0; lots_of_threads--) {
        res = pthread_join(a_thread[lots_of_threads], &thread_result);
        if (res == 0)
	        printf ("Picked up a thread\n");
        else
	        perror ("pthread_join failed");
    }
    
    printf("All done\n");
    exit(EXIT_SUCCESS);
}

void *thread_function(void *arg) {
    int my_number = *(int *) arg;
    // int rand_num;
    printf("Thread_function is running. Argument was %d\n", my_number);
    // rand_num = 1 + (int) (9.0 * rand () / (RAND_MAX + 1.0));
    printf("Bye from %d\n", my_number);
    pthread_exit(NULL);
}
