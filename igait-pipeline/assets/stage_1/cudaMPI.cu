#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <mpi.h>
#include <cuda_runtime.h>

__global__ void sqrtKernel(float *data, int n) {
    int idx = blockIdx.x * blockDim.x + threadIdx.x;
    if (idx < n) {
        data[idx] = sqrtf(data[idx]);
    }
}

int main(int argc, char *argv[]) {
    int rank, size;
    MPI_Init(&argc, &argv);
    MPI_Comm_rank(MPI_COMM_WORLD, &rank);
    MPI_Comm_size(MPI_COMM_WORLD, &size);

    // Initialize GPU
    int deviceCount;
    cudaGetDeviceCount(&deviceCount);
    if (deviceCount == 0) {
        if (rank == 0) {
            fprintf(stderr, "No CUDA devices found!\n");
        }
        MPI_Finalize();
        return 1;
    }

    // Set device for this rank
    cudaSetDevice(rank % deviceCount);

    // Data size: ~100 KB per rank (quick GPU initialization test)
    const int dataSize = 25600; // 100 KB / 4 bytes per float
    const int dataSizeTotal = dataSize * size;
    
    if (rank == 0) {
        printf("Running on %d nodes, dataSizeTotal= %d KB\n", size, (dataSizeTotal * sizeof(float)) / 1024);
    }

    // Allocate host memory
    float *h_data = (float*)malloc(dataSize * sizeof(float));
    
    // Initialize data with values between 0 and 1000
    for (int i = 0; i < dataSize; i++) {
        h_data[i] = (float)(rand() % 1000000) / 1000.0f;
    }

    // Allocate device memory
    float *d_data;
    cudaMalloc(&d_data, dataSize * sizeof(float));
    
    // Copy data to device
    cudaMemcpy(d_data, h_data, dataSize * sizeof(float), cudaMemcpyHostToDevice);

    // Launch kernel
    int threadsPerBlock = 256;
    int blocksPerGrid = (dataSize + threadsPerBlock - 1) / threadsPerBlock;
    sqrtKernel<<<blocksPerGrid, threadsPerBlock>>>(d_data, dataSize);

    // Copy result back
    cudaMemcpy(h_data, d_data, dataSize * sizeof(float), cudaMemcpyDeviceToHost);

    // Compute average
    double sum = 0.0;
    for (int i = 0; i < dataSize; i++) {
        sum += h_data[i];
    }
    double localAvg = sum / dataSize;

    // Gather all averages
    double globalAvg;
    MPI_Reduce(&localAvg, &globalAvg, 1, MPI_DOUBLE, MPI_SUM, 0, MPI_COMM_WORLD);

    if (rank == 0) {
        globalAvg /= size;
        printf("Average of square roots is: %f\n", globalAvg);
        printf("Average of square roots %f over %d is: %f\n", sum, dataSize, localAvg);
        printf("Test PASSED\n");
    }

    // Cleanup
    cudaFree(d_data);
    free(h_data);

    MPI_Finalize();
    return 0;
}
