package ua.kpi.structures.graphs;
import ua.kpi.structures.lists.List;

import java.util.NoSuchElementException;

public class Graph {
    private final int V;
    private int E;
    private List<Edge>[] adj; // adjacency

    public Graph(int V) {
        if (V < 0) throw new IllegalArgumentException("number of vertices have to be > 0");
        this.V = V;
        this.E = 0;
        adj = (List<Edge>[]) new List[V];
        for (int v = 0; v < V; v++) {
            adj[v] = new List<>();
        }
    }

    private void validateVertex(int v) {
        if (v < 0 || v >= V) throw new IndexOutOfBoundsException("vertex " + v + " is not between 0 and " + (V-1));
    }

    public int V() {
        return V;
    }

    public int E() {
        return E;
    }

    public void addEdge(Edge e) {
        int v = e.either(); validateVertex(v);
        int w = e.other(v); validateVertex(w);
        adj[v].push(e);
        adj[w].push(e);
        E++;
    }

    public void deleteEdge(int v, int w) {
        for (Edge e : adj[v]) {
            if (e.other(v) == w) {
                adj[v].deleteItem(e);
                adj[w].deleteItem(e);
                return;
            }
        }
        throw new NoSuchElementException("no edges with such vertices");
    }

    public int degree(int v) {
        validateVertex(v);
        return adj[v].size();
    }

    public Iterable<Edge> adj(int v) {
        validateVertex(v);
        return adj[v];
    }

    public Iterable<Edge> edges() {
        List<Edge> edges = new List<>();
        for (int v = 0; v < V; v++) {
            for (Edge e : adj[v]) {
                if (e.other(v) > v) edges.push(e);
            }
        }
        return edges;
    }

    public String toString() {
        StringBuilder s = new StringBuilder();
        s.append(V + " vertices, " + E + " edges\n");
        for (int v = 0; v < V; v++) {
            s.append(v + ": ");
            for (Edge e : adj[v]) {
                s.append(e + "  ");
            }
            s.append("\n");
        }
        return s.toString();
    }
}
