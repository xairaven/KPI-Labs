#include <dirent.h>
#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <string.h>

// Task: to write simple implementation of ls function
// example of usage: (input - command prompt)
// /home/xairaven/KPI/KPI-Labs
int main(int argc, char *argv[]) {
    DIR *dp;
    struct dirent *dirp;

    if (argc != 2) {
        fprintf(stderr, "Usage: ls folder_name\n");
        exit(1);
    }

    if (!(dp = opendir(argv[1]))) {
        fprintf(stderr, "Unable to open %s: %s\n", argv[1], strerror(errno));
        exit(1);
    }


    printf("Files and folders in folder %s:\n", argv[1]);
    while ((dirp = readdir(dp))) {
        printf("- \t%s\n", dirp->d_name);
    }

    closedir(dp);
    exit(0);
}