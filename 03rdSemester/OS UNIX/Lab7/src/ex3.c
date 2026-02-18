#include <stdio.h>
#include <pthread.h>

pthread_mutex_t count_mutex;
long count;

void* increment_count(void *arg) {
    pthread_mutex_lock(&count_mutex);
    count = count + 1;
    pthread_mutex_unlock(&count_mutex);
    
    return NULL;
}

long get_count() {
    long c;
    
    pthread_mutex_lock(&count_mutex);
    c = count;
    pthread_mutex_unlock(&count_mutex);
    
    return c;
}

int main() {
    pthread_t thread[5];
    pthread_mutex_init(&count_mutex, NULL);
    for (int i = 0; i < 5; i++) {
        pthread_create(&(thread[i]), NULL, increment_count, NULL);
        printf("%ld\n", get_count());
    }
    printf("\n");
    return 0;
}

