#include <stdio.h>
#include <pthread.h>

#define LIMIT 16
pthread_mutex_t count_mutex;
long count = 0;

void *increment_count(void *arg) {
    pthread_mutex_lock(&count_mutex);
    count = count + 1;
    printf("%ld\n", count);
    pthread_mutex_unlock(&count_mutex);
	
    return NULL;
}

int main(void) {
	pthread_t a_thread[LIMIT];
	pthread_mutex_init(&count_mutex, NULL);
	
    for (register int i = 0; i < LIMIT; i++) {
	    pthread_create(&(a_thread[i]),NULL, increment_count, NULL);
        //printf("%ld\n", get_count());
	}

	for(register int i = 0; i < LIMIT; i++) {
	    pthread_join(a_thread[i], NULL);
	}

	pthread_mutex_destroy(&count_mutex);
	
    return 0;
}
