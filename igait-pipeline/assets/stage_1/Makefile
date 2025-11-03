NVCC = nvcc
MPICC = mpicc
TARGET = cudaMPI

# CUDA flags
NVCCFLAGS = -O3 -arch=sm_80
# MPI flags (get from mpi compiler wrapper)
MPIFLAGS = $(shell mpicc -showme:compile | sed 's/-pthread/-Xcompiler -pthread/g')
# Wrap linker flags with -Xlinker, except -L and -l flags
MPILIBS = $(shell mpicc -showme:link | sed 's/-pthread/-Xcompiler -pthread/g' | sed 's/-Wl,/-Xlinker /g')

all: $(TARGET)

$(TARGET): cudaMPI.cu
	$(NVCC) $(NVCCFLAGS) $(MPIFLAGS) -o $(TARGET) cudaMPI.cu $(MPILIBS)

clean:
	rm -f $(TARGET)

.PHONY: all clean
