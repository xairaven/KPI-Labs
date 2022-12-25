#include <stdlib.h>
#include <stdio.h>
#include <pthread.h>

void* thread_func(void *arg) {
    pthread_setcancelstate(PTHREAD_CANCEL_DISABLE, NULL);
    for (int i = 0; i < 4; i++) {
        printf ("I'm still running!\n");
    }
    pthread_setcancelstate(PTHREAD_CANCEL_ENABLE, NULL);
    pthread_testcancel();
    printf("YOU WILL NOT STOP ME!!!\n");
    return NULL;
}

int main(int argc, char *argv[]) {
    pthread_t thread;
    pthread_create(&thread, NULL, thread_func, NULL);
    pthread_cancel(thread);
    printf("Requested to cancel the thread\n");
    pthread_join(thread, NULL);
    printf("The thread is stopped.\n");
    return EXIT_SUCCESS;
}
