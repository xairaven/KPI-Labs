#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define BUFFSIZE 4096

int main(void) {
    int n;
    char buf[BUFFSIZE];

    while ((n = read(STDIN_FILENO, buf, BUFFSIZE)) > 0)
        if (write(STDOUT_FILENO, buf, n) != n){
            fprintf(stderr, "Writing error: %s\n", strerror(errno));
            exit(1);
        }

    if (n < 0) {
        fprintf(stderr, "Reading error: %s\n", strerror(errno));
        exit(1);
    }

    exit(0);
}