#include <stdio.h>
#include <pthread.h>
#include <signal.h>
#include <unistd.h>
#include <sys/syscall.h>

#define NUM_THREADS 5

// Array for storing thread ids
pthread_t threads[NUM_THREADS];

// Callback for all threads
void *threadFunction(void *threadID) {
    long tid = (long)threadID;

    printf("Thread %ld is waiting for SIGINT...\n", tid);

    // Set for sigwait
    sigset_t set;
    sigemptyset(&set);
    sigaddset(&set, SIGINT);

    // Waiting for SIGINT
    int received_signal;
    sigwait(&set, &received_signal);

    pid_t local_pid = syscall(SYS_gettid);
    printf("Thread %ld (PID %ld) received the signal: %d\n", tid, (long) local_pid, received_signal);

    pthread_exit(NULL);
}

int main() {
    // Blocking SIGINT for the main thread
    sigset_t main_set;
    sigemptyset(&main_set);
    sigaddset(&main_set, SIGINT);
    pthread_sigmask(SIG_BLOCK, &main_set, NULL);

    // Creating threads
    for (long i = 0; i < NUM_THREADS; i++) {
        if (pthread_create(&threads[i], NULL, threadFunction, (void *)i) != 0) {
            fprintf(stderr, "Error creating thread %ld\n", i);
            return 1;
        }
    }

    // Waiting for SIGINT in the main thread
    printf("Main thread is waiting for SIGINT...\n");
    int received_signal;
    sigwait(&main_set, &received_signal);

    printf("Main thread received the signal: %d\n", received_signal);

    // Exiting from all threads
    for (long i = 0; i < NUM_THREADS; i++) {
        if (pthread_join(threads[i], NULL) != 0) {
            fprintf(stderr, "Error joining thread %ld\n", i);
            return 1;
        }
    }

    return 0;
}