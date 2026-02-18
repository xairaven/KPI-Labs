#include <pthread.h>
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <unistd.h>

#define SIZE_I 2
#define SIZE_J 2

float X[SIZE_I][SIZE_J];
float S[SIZE_I][SIZE_J];

int count = 0; // глобальний лічильник

struct DATA_ {
    double x;
    int i;
    int z;
};

typedef struct DATA_ DATA;

// Оголошення змінної м'ютексу
pthread_mutex_t lock;

// Функція для обчислень
double f(double x) { return x*x; }

// Потокова функція для обчислень
void *calc_thr(void *arg) {
    DATA* a = (DATA*) arg;
    X[a->i][a->z] = f(a->x);

    // установка блокування
    pthread_mutex_lock(&lock);

    // зміна глобальної змінної
    count++;

    // зняття блокування  
    pthread_mutex_unlock(&lock);
    free(a);
    
    return NULL;
}

// Потокова функція для введення
void *input_thr(void *arg) {
    DATA* a = (DATA*) arg;
    
    pthread_mutex_lock(&lock);
    
    printf("S[%d][%d]: ", a->i, a->z);
    scanf("%f", &S[a->i][a->z]);

    pthread_mutex_unlock(&lock);

    free(a);

    return NULL;
}

int main() {
    //масив ідентифікаторів потоків
    pthread_t thr[SIZE_I * SIZE_J];

    // ініціалізація м’ютекса
    pthread_mutex_init(&lock, NULL);

    DATA *arg;

    // Введення даних для обробки
    for (int i = 0; i < SIZE_I; i++) {
        for (int z = 0; z < SIZE_J; z++) {
            arg = malloc(sizeof(DATA));
            arg->i = i;
            arg->z = z;

            // створення потоку для введення елементів матриці
            pthread_create(&thr[i * SIZE_J + z], NULL, input_thr, (void*) arg);
        }
    }

    // Очікування завершення усіх потоків введення даних
    // ідентифікатори потоків зберігаються у масиві thr

    for (int i = 0; i < SIZE_I * SIZE_J; i++) {
        pthread_join (thr[i], NULL);
    }

    // Обчислення елементів матриці
    pthread_t thread;
    printf("Start calculation\n");
    for (int i = 0; i < SIZE_I; i++) {
        for (int z = 0; z < SIZE_J; z++) {
            arg = malloc(sizeof(DATA));
            arg->i = i;
            arg->z = z;
            arg->x = S[i][z];
            
            // створення потоку для обчислень
            pthread_create (&thread, NULL, calc_thr, (void *) arg);

            // переведення потоку у режим від’єнання
            pthread_detach(thread);
        }   
    }

    do {
        // Основний процес "засинає" на 1с
        // Перевірка стану обчислень
        sleep(1);
        printf("Finished %d threads.\n", count);
    } while (count < SIZE_I * SIZE_J);

    // Виведення результатів
    for (int i = 0; i < SIZE_I; i++) {
        for (int z = 0; z < SIZE_J; z++) {
            printf("X[%d][%d] = %f\t", i, z, X[i][z]);
        } 
        printf("\n");
    }

    // видалення м’ютекса
    pthread_mutex_destroy(&lock);

    return 0;
}
