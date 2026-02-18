#include <stdio.h>
#include <sys/syscall.h>
#include <pthread.h>
#include <unistd.h>

#define NUM_THREADS 5

// Function executed by every thread
void *threadFunction(void *threadID) {
    long tid = (long) threadID;
    pid_t local_pid = syscall(SYS_gettid);
    printf("Thread number: %ld | PID: %d | PID_SELF: %lu | LWP_ID: %ld | PGID: %d\n", tid, getpid(), pthread_self(), (long) local_pid, getpgrp());
    pthread_exit(NULL);
}

int main() {
    // Threads ids
    pthread_t threads[NUM_THREADS];

    // Creating threads
    for (long i = 0; i < NUM_THREADS; i++) {
        if (pthread_create(&threads[i], NULL, threadFunction, (void *)i) != 0) {
            fprintf(stderr, "Error creating thread %ld\n", i);
            return 1;
        }
    }

    while(1) {
        sleep(1);
    }

    // Waiting for each thread to complete
    for (long i = 0; i < NUM_THREADS; i++) {
        if (pthread_join(threads[i], NULL) != 0) {
            fprintf(stderr, "Error joining thread %ld\n", i);
            return 1;
        }
    }

    return 0;
}