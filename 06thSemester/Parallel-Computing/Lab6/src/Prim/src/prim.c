#include "mpi.h"
#include <stdio.h>
#include <math.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>

typedef struct DWeight {
  int weight;
  int node;
} DWeight;

typedef struct DEdge {
  int fromNode;
  int toNode;
  int weight;
} DEdge;

void fscanfEdgeList(FILE* file, int **adMatrix, int *nodesNmb) {
  int edgesNmb, node1, node2, weight, matrixSize, nodes;

  fscanf(file, "nodes-%d,edges-%d,weights", &nodes, &edgesNmb);
  *nodesNmb = nodes;

  matrixSize = nodes * nodes;
  *adMatrix = (int*) malloc(matrixSize * sizeof(int));
  
  for (int i = 0; i < matrixSize; (*adMatrix)[i] = 0, i++);

  for (int i = 0; i < edgesNmb; i++) {
      fscanf(file, "%d,%d,%d", &node1, &node2, &weight);
      (*adMatrix)[node1 * nodes + node2] = weight;
      (*adMatrix)[node2 * nodes + node1] = weight;
  }
}

void fprintfAdMatrix(FILE* file, int* adMatrix, int rowNmb, int colNmb) {
  int index = 0;
  for (int row = 0; row < rowNmb; row++) {
    for (int column = 0; column < colNmb; column++,index++) {
      fprintf(file, "%d ", adMatrix[index]);
    }
    fprintf(file, "\n");
  }
}

void fprintfDTable(FILE* file, DWeight* dTable, int rowNmb, int processId) {
  for (int row = 0; row < rowNmb; row++) {
    fprintf(file, "%d: %d|%d,%d\n", processId, row, dTable[row].node, dTable[row].weight);
  }
}

void fprintfDEdges(FILE* file, DEdge* edes, int nodeNmb, int rowNmb, double time_taken) {
  fprintf(file, "nodes-%d,edges-%d,weights\n", nodeNmb, rowNmb);
  for (int row = 0; row < rowNmb; row++) {
    fprintf(file, "%d,%d,%d\n", edes[row].fromNode, edes[row].toNode, edes[row].weight);
  }
  fprintf(file, "Done in: %.6f secs\n", time_taken);
}

void primPartitionMatrix(int *adMatrixFull, int nodesNmb, int processId, int processNmb, int **adMatrixPartial, int *nodesProcesNmb, int *startNode) {

  int matrixSize = nodesNmb * nodesNmb;
  int lastId = processNmb - 1;

  int middleNodes = (int) ceil((float) nodesNmb / (float) processNmb);
  int middleSize = nodesNmb * middleNodes;

  int lastNodes = nodesNmb - (middleNodes * lastId);
  int lastSize = matrixSize - lastId * middleSize;
  int lastCommProcessNmb, lastCommProcessId;

  MPI_Comm lastComm;
  
  *startNode = middleNodes * processId;

  if (processNmb > 1) {
    MPI_Comm_split(MPI_COMM_WORLD, processId / lastId, processId, &lastComm);
    MPI_Comm_size(lastComm, &lastCommProcessNmb);
    MPI_Comm_rank(lastComm, &lastCommProcessId);

    if (processId != lastId) {
      *adMatrixPartial = (int*) malloc(middleSize * sizeof(int));
      *nodesProcesNmb = middleNodes;

      MPI_Scatter(adMatrixFull, middleSize, MPI_INT, 
                 *adMatrixPartial, middleSize, MPI_INT, 
                 0, lastComm);
    } else {
      *adMatrixPartial = (int*) malloc(lastSize * sizeof(int));
      *nodesProcesNmb = lastNodes;
    }

    if (processId == 0) {
      MPI_Send(adMatrixFull + (lastId * middleSize), lastSize, MPI_INT, lastId, 0, MPI_COMM_WORLD);
    }
    else if (processId == lastId){
      MPI_Recv(*adMatrixPartial, lastSize, MPI_INT, 0, 0, MPI_COMM_WORLD, MPI_STATUS_IGNORE);
    }

  } else {
    *adMatrixPartial = (int*) malloc(matrixSize * sizeof(int));
    *nodesProcesNmb = nodesNmb;
    memcpy(*adMatrixPartial, adMatrixFull, matrixSize * sizeof(int));
  }
}

void primPartitionDArray(int nodesNmbProcess, int nodesNmb, int firstNode, int* adMatrixPartial, DWeight** dTable) {
  int weight;
  *dTable = (DWeight*) malloc(nodesNmbProcess * sizeof(DWeight));

  for (int row = 0; row < nodesNmbProcess; row++) {
    weight = adMatrixPartial[row * nodesNmb + firstNode];
    (*dTable)[row].weight = weight > 0 ? weight : INT_MAX;
    (*dTable)[row].node   = firstNode;
  }
}

void primFindMinimum(int startNode, int nodesNmbProcess, DWeight* dTable, int* isAdded, DWeight *local) {
  int globalNode  = 0;
  int localWeight = 0;
  local->weight   = INT_MAX;
  local->node     = -1;

  for (int localNode = 0; localNode < nodesNmbProcess; localNode++) {
    globalNode = localNode + startNode;

    if (!isAdded[globalNode]) {

      localWeight = dTable[localNode].weight;
      
      if (localWeight != 0 && (local->node == -1 || local->weight > localWeight)) {
        local->weight = localWeight;
        local->node   = globalNode;
      }
    }
  }
}

void primBroadcastSolution(int startNode, int nodesNmbProcess, DWeight *dTable, DWeight globalMin, DEdge* edge) {
  // Function brodcasts solution to every process. 
  int fromNode = -1, fromNodeGlobal;
  MPI_Bcast(&globalMin, 1, MPI_2INTEGER, 0, MPI_COMM_WORLD);

  // Finding responisble partition
  if (startNode <= globalMin.node && globalMin.node < startNode + nodesNmbProcess) {
    // Only one process have fromNode value greater then -1
    fromNode = dTable[globalMin.node - startNode].node;
  }

  // FromNode value cannot be brodcasted along toNode,value tuple so separate
  // Reduce and broadcast is needed
  MPI_Reduce(&fromNode, &fromNodeGlobal, 1, MPI_INT, MPI_MAX, 0, MPI_COMM_WORLD);
  MPI_Bcast(&fromNodeGlobal, 1, MPI_INT, 0, MPI_COMM_WORLD);

  // Saving solution in every process
  edge->weight    = globalMin.weight;
  edge->toNode    = globalMin.node;
  edge->fromNode  = fromNodeGlobal;
}

void primUpdateDArray(int *adMatrixPartial, int nodesNmb, int nodesNmbProcess, DEdge *edge, int *isAdded, DWeight *dTable) {
  // Function updates D array after adding new node to tree

  int newWeight = 0;
  isAdded[edge->toNode] = 1;

  for (int row = 0; row < nodesNmbProcess; row++) {
    // Weight from new node
    newWeight = adMatrixPartial[row * nodesNmb + edge->toNode];

    // Check if new path to node is better then previous one 
    if (dTable[row].weight > newWeight && newWeight > 0) {
      dTable[row].node    = edge->toNode;
      dTable[row].weight  = newWeight;
    }
  }
}

void primAlgorithm(int *adMatrix, int nodesNmb, int processId, int processNmb, DEdge* edges) {
  // Prim's Algorithm 

  int     *adMatrixPartial = 0;     // chunk of adMatrix 
  int     *isNodeAdded = 0;         // stores 1 if node is already in tree
  int     nodesNmbProcess = 0;      // nodes per process
  int     firstNode = 0;            // algorithm start node
  int     edgesNmb = nodesNmb - 1;  // edge count
  int     startNode = 0;            // start node number for partition

  DWeight *dTable = 0;              // weights array
  DWeight localMin;
  DWeight globalMin;

  // Partitioning of adjacency matrix and distance array
  primPartitionMatrix(adMatrix, nodesNmb, processId, processNmb, &adMatrixPartial, &nodesNmbProcess, &startNode);
  primPartitionDArray(nodesNmbProcess, nodesNmb, firstNode, adMatrixPartial, &dTable);

  // Algorithm initialization
  isNodeAdded = (int*) malloc(nodesNmb * sizeof(int));
  for (int i = 0; i < nodesNmb; isNodeAdded[i] = 0,i++);
  isNodeAdded[firstNode] = 1;

  // Main iteration
  for (int index = 0; index < edgesNmb; index++) {

    DEdge *edge = &edges[index];

    // 1. Each process P_i computes d_i = min{dTable}
    primFindMinimum(startNode, nodesNmbProcess, dTable, isNodeAdded, &localMin);
    
    // 2. Global minimum d is then obtained by using all-to-one reduction operation
    //    and its stored in P_0 process. P_0 stores new vortex u
    MPI_Reduce(&localMin, &globalMin, 1, MPI_2INTEGER, MPI_MINLOC, 0, MPI_COMM_WORLD);
    
    // 3. Process P_0 broadcasts u one-to-all. The process responsible for u 
    //    marks u as belonging to tree.
    primBroadcastSolution(startNode, nodesNmbProcess, dTable, globalMin, edge);

    // 4. Each process updates the values od d[v] for its local vertices 
    primUpdateDArray(adMatrixPartial, nodesNmb, nodesNmbProcess, edge, isNodeAdded, dTable);
  }

  free(isNodeAdded);
  free(adMatrixPartial);
  free(dTable);
}

int main( int argc, char *argv[] )
{
  FILE   *file;
  char   *filename = argv[1];
  char   *output_filename = argv[2];
  double time_taken;
  int    processId, processNmb;
  int    nodesNmb = 0;
  int    *adMatrix = 0;
  DEdge  *edges;

  // MPI Initialization
  MPI_Init(&argc, &argv);
  MPI_Comm_size(MPI_COMM_WORLD, &processNmb);
  MPI_Comm_rank(MPI_COMM_WORLD, &processId);

  // Read edge list from file
	if (processId == 0) {
    file = fopen(filename, "r");
    if (file == 0) {
      printf("Cannot found input file %s!\n", filename);
      MPI_Abort(MPI_COMM_WORLD, 1);
    }
		fscanfEdgeList(file, &adMatrix, &nodesNmb);
    fclose(file);
	}

  // Initialization
  MPI_Bcast(&nodesNmb, 1, MPI_INT, 0, MPI_COMM_WORLD);
  edges = (DEdge*) malloc((nodesNmb - 1) * sizeof(DEdge));

  // Starts Timer
  time_taken -= MPI_Wtime();

  // Prim's Algorithm
  primAlgorithm(adMatrix, nodesNmb, processId, processNmb, edges);
  
  // Stop the timer
  time_taken += MPI_Wtime();

  // Free allocated resources
  if(processId == 0) {
	if (argc == 2) {
		fprintfDEdges(stdout, edges, nodesNmb, nodesNmb - 1, time_taken);
	} else if (argc == 3) {
		FILE *file_ptr = fopen(output_filename, "w");

		if (file_ptr != NULL) {
			fprintfDEdges(file_ptr, edges, nodesNmb, nodesNmb - 1, time_taken);
			fclose(file_ptr);
		} else {
			printf("Error opening file %s\n", output_filename);
			free(adMatrix);
			free(edges);
			MPI_Finalize();
			return 0;
		}
	}
   
    free(adMatrix);
  }
  free(edges);

  MPI_Finalize();
  return 0;
}
