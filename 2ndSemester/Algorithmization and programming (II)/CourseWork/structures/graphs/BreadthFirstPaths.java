package ua.kpi.structures.graphs;
import ua.kpi.structures.lists.Queue;
import ua.kpi.structures.lists.List;

public class BreadthFirstPaths {
    private static final int INFINITY = Integer.MAX_VALUE;
    private boolean[] marked;
    private int[] distTo;
    private Edge[] edgeTo;
    private final int s;

    public BreadthFirstPaths(Graph G, int s) {
        marked = new boolean[G.V()];
        distTo = new int[G.V()];
        edgeTo = new Edge[G.V()];
        this.s = s;
        bfs(G, s);
    }

    private void bfs(Graph G, int s) {
        Queue<Integer> q = new Queue<>();
        for (int v = 0; v < G.V(); v++) distTo[v] = INFINITY;
        distTo[s] = 0;
        marked[s] = true;
        q.enqueue(s);
        while(!q.isEmpty()) {
            int v = q.dequeue();
            for (Edge e : G.adj(v)) {
                int w = e.other(v);
                if (!marked[w]) {
                    edgeTo[w] = e;
                    distTo[w] = distTo[v] + 1;
                    marked[w] = true;
                    q.enqueue(w);
                }
            }
        }
    }

    public boolean hasPathTo(int v) {
        return marked[v];
    }

    public int distTo(int v) {
        return distTo[v];
    }

    public Iterable<Integer> pathTo(int v) {
        if (!hasPathTo(v)) return null;
        List<Integer> path = new List<>();
        for (int x = v; x != s; x = edgeTo[x].other(x)) {
            path.push(x);
        }
        path.push(s);
        return path;
    }
}

