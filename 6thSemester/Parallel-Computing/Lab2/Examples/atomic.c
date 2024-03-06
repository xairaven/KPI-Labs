int sum_arr(int *a, const int n) {
    int sum = 0;

    #pragma omp parallel
    {
        int local_sum = 0;

        #pragma omp for
        for (int i = 0; i < n; i++)
            local_sum += a[i];

        #pragma omp atomic
        sum += local_sum;
    }
    return sum;
}
