#include <iostream>
#include <math.h>

// Kernel function to add the elements of two arrays
__global__
void add(int n, float *x, float *y)
{
  int index = blockIdx.x * blockDim.x + threadIdx.x;
  int stride = blockDim.x * gridDim.x;
  for (int i = index; i < n; i += stride)
    y[i] = x[i] + y[i];
}

int main(void)
{
    int N = 1<<20;
    float *x, *y;

    // Allocate Unified Memory accessible from CPU or GPU
    cudaError_t errX = cudaMallocManaged(&x, N*sizeof(float));
    cudaError_t errY = cudaMallocManaged(&y, N*sizeof(float));

    if (errX != cudaSuccess || errY != cudaSuccess) {
        std::cerr << "Memory allocation failed" << std::endl;
        return -1;
    }

    // Initialize x and y arrays on the host
    for (int i = 0; i < N; i++) {
        x[i] = 1.0f;
        y[i] = 2.0f;
    }

    // Run kernel on 1M elements on the GPU
    int blockSize = 256;
    int numBlocks = (N + blockSize - 1) / blockSize;
    add<<<numBlocks, blockSize>>>(N, x, y);

    // Check for errors immediately after kernel launch
    cudaError_t errLaunch = cudaGetLastError();
    if (errLaunch != cudaSuccess) {
        std::cerr << "CUDA Error after launch: " << cudaGetErrorString(errLaunch) << std::endl;
    }

    // Wait for GPU to finish before accessing on host
    cudaError_t errSync = cudaDeviceSynchronize();
    if (errSync != cudaSuccess) {
        std::cerr << "CUDA Error during sync: " << cudaGetErrorString(errSync) << std::endl;
    }

    // Check for errors
    float maxError = 0.0f;
    for (int i = 0; i < N; i++) {
        maxError = fmax(maxError, fabs(y[i]-3.0f));
    }
    std::cout << "Max error: " << maxError << std::endl;

    // Free memory
    cudaFree(x);
    cudaFree(y);
    return 0;
}


