#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>
#include <signal.h>
#include <sys/syscall.h>
#include <unistd.h>

void sigaction_handler(int signo) {
    printf("Received SIGINT using sigaction in thread %ld\n", (long) syscall(SYS_gettid));
    pthread_exit(NULL);
}

void *sigaction_thread(void *arg) {
    // Initializing sigaction handler
    struct sigaction sa;
    sa.sa_handler = sigaction_handler;
    sigaction(SIGINT, &sa, NULL);

    // Blocking thread while waiting for a signal
    pause();

    printf("Exiting sigaction_thread\n");

    return NULL;
}

void *sigwait_thread(void *arg) {
    sigset_t set;
    int sig;

    // Initializing set of signals that threads are waiting
    sigemptyset(&set);
    sigaddset(&set, SIGINT);

    // Waiting for this signal
    sigwait(&set, &sig);

    printf("Received SIGINT using sigwait in thread %ld\n", (long) syscall(SYS_gettid));

    pthread_exit(NULL);

    return NULL;
}

int main() {
    pthread_t sigaction_thread_id, sigwait_thread_id;

    printf("Waiting for SIGINT (Ctrl+C):\n");
    fflush(stdout);

    // Creating threads
    pthread_create(&sigaction_thread_id, NULL, sigaction_thread, NULL);
    pthread_create(&sigwait_thread_id, NULL, sigwait_thread, NULL);

    // Waiting until threads will be done
    pthread_join(sigaction_thread_id, NULL);
    pthread_join(sigwait_thread_id, NULL);

    return 0;
}