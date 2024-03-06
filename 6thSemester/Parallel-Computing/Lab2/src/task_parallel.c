#include <stdio.h>
#include <omp.h>

double f(double x, double y, double z) {
    return x*x*x*x * y*y*y*y * z*z*z*z;
}

// 300, 200, 400
int main() {
    const int ni = 600, nj = 400, nk = 800;
    const double a = 0.0, b = 1.0;
    const double dx = (b - a) / ni;
    const double dy = (b - a) / nj;
    const double dz = (b - a) / nk;
    double sum = 0.0;

    #pragma omp parallel for reduction(+:sum)
    for (int i = 0; i < ni; ++i) {
        for (int j = 0; j < nj; ++j) {
            for (int k = 0; k < nk; ++k) {
                double x = a + i * dx;
                double y = a + j * dy;
                double z = a + k * dz;
                double result = f(x, y, z) * dx * dy * dz;
                sum += result;
            }
        }
    }

    double integral = sum;
    printf("Approximate integral value: %lf\n", integral);

    return 0;
}
