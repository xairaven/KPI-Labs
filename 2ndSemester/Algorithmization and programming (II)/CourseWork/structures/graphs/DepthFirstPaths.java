package ua.kpi.structures.graphs;
import ua.kpi.structures.lists.List;

public class DepthFirstPaths {
    private boolean[] marked;
    private Edge[] edgeTo;
    private final int s;

    public DepthFirstPaths(Graph G, int s) {
        this.s = s;
        edgeTo = new Edge[G.V()];
        marked = new boolean[G.V()];
        dfp(G, s);
    }

    private void dfp(Graph G, int v) {
        marked[v] = true;
        for (Edge e : G.adj(v)) {
            int w = e.other(v);
            if (!marked[w]) {
                edgeTo[w] = e;
                dfp(G, w);
            }
        }
    }

    public boolean hasPathTo(int v) {
        return marked[v];
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
