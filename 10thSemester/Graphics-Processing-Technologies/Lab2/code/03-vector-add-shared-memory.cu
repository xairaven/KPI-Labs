#include <stdio.h>

// Kernel to initialize array elements
__global__
void initWith(float num, float *a, int N)
{
  int index = threadIdx.x + blockIdx.x * blockDim.x;
  int stride = blockDim.x * gridDim.x;

  for(int i = index; i < N; i += stride)
  {
    a[i] = num;
  }
}

// Kernel that uses shared memory as a tile cache
__global__
void addVectorsIntoShared(float *result, float *a, float *b, int N)
{
  // Allocate dynamic shared memory for the current block
  extern __shared__ float shared_mem[];
  
  // Split shared memory into two logical arrays for inputs
  float* s_a = shared_mem;
  float* s_b = &shared_mem[blockDim.x];

  int index = threadIdx.x + blockIdx.x * blockDim.x;
  int stride = blockDim.x * gridDim.x;
  int tid = threadIdx.x;

  for(int i = index; i < N; i += stride)
  {
    // Load elements from global memory to shared memory cache
    s_a[tid] = a[i];
    s_b[tid] = b[i];

    // Synchronize to ensure all elements are loaded
    __syncthreads();

    // Compute the sum using cached data
    float temp_sum = s_a[tid] + s_b[tid];

    // Synchronize before writing and moving to the next iteration
    __syncthreads();

    // Write the computed sum back to global memory
    result[i] = temp_sum;
  }
}

// Function to verify the results
void checkElementsAre(float target, float *vector, int N)
{
  for(int i = 0; i < N; i++)
  {
    if(vector[i] != target)
    {
      printf("FAIL: vector[%d] - %0.0f does not equal %0.0f\n", i, vector[i], target);
      exit(1);
    }
  }
  printf("Success! All values calculated correctly.\n");
}

int main()
{
  int deviceId;
  int numberOfSMs;

  cudaGetDevice(&deviceId);
  cudaDeviceGetAttribute(&numberOfSMs, cudaDevAttrMultiProcessorCount, deviceId);

  const int N = 2<<24;
  size_t size = N * sizeof(float);

  float *c = (float *) malloc(size);

  float *da;
  float *db;
  float *dc;
  
  cudaMalloc(&da, size);
  cudaMalloc(&db, size);
  cudaMalloc(&dc, size);

  size_t threadsPerBlock = 256;
  size_t numberOfBlocks = 32 * numberOfSMs;

  initWith<<<numberOfBlocks, threadsPerBlock>>>(3, da, N);
  initWith<<<numberOfBlocks, threadsPerBlock>>>(4, db, N);
  initWith<<<numberOfBlocks, threadsPerBlock>>>(0, dc, N);

  // Calculate required shared memory size for two arrays per block
  size_t sharedMemSize = 2 * threadsPerBlock * sizeof(float);

  addVectorsIntoShared<<<numberOfBlocks, threadsPerBlock, sharedMemSize>>>(dc, da, db, N);

  cudaError_t addVectorsErr = cudaGetLastError();
  if(addVectorsErr != cudaSuccess) printf("Error: %s\n", cudaGetErrorString(addVectorsErr));

  cudaError_t asyncErr = cudaDeviceSynchronize();
  if(asyncErr != cudaSuccess) printf("Error: %s\n", cudaGetErrorString(asyncErr));

  cudaMemcpy(c, dc, size, cudaMemcpyDeviceToHost);

  checkElementsAre(7, c, N);

  cudaFree(da);
  cudaFree(db);
  cudaFree(dc);
  
  free(c);
  return 0;
}
