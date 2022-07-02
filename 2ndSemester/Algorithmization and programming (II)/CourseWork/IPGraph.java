package ua.kpi;

import java.util.NoSuchElementException;

import ua.kpi.parser.In;
import ua.kpi.structures.graphs.*;
import ua.kpi.structures.lists.DynamicArray;
import ua.kpi.structures.lists.List;
import ua.kpi.structures.trees.RedBlackBST;

public class IPGraph {
    private DynamicArray<String> IPs;
    private RedBlackBST<String, List<String>> st;
    private Graph graph;

    public IPGraph(String domains, String connections) {
        int vertices = 0;
        In IPFile = new In(domains);
        IPs = new DynamicArray<>();                                  // ArrayList for indexes
        st = new RedBlackBST<>();

        while (IPFile.hasNextLine()) {
            String[] temp = IPFile.readLine().split(",");   // [0] - domain, [1] - ip (KEY OF BST)
            if (!st.contains(temp[1])) {
                st.put(temp[1], new List<>());
                vertices++;
            }
            st.get(temp[1]).push(temp[0]);
            if (!IPs.contains(temp[1])) IPs.add(temp[1]);
        }

        In connectionsIn = new In(connections);
        graph = new Graph(vertices);
        while (connectionsIn.hasNextLine()) {
            String[] temp = connectionsIn.readLine().split(",");
            if (!IPs.contains(temp[0]) || !IPs.contains(temp[1])) {
                throw new IllegalArgumentException("connecting IPs that aren't in domains list");
            }

            int weight = (int) ((Math.random() * 40) + 10);
            graph.addEdge(new Edge(IPs.indexOf(temp[0]), IPs.indexOf(temp[1]), weight));
        }
    }

    public String domainIPList() {
        StringBuilder sb = new StringBuilder();
        sb.append("|----------------------------------------------|\n");
        sb.append(String.format("| %-20s -> %20s |\n", "Domains", "IP"));
        sb.append("|----------------------------------------------|\n");
        for (String IP : st.keys()) {
            for (String domain : st.get(IP)) {
                sb.append(String.format("| %-20s -> %20s |\n", domain, IP));
            }
        }
        sb.append("|----------------------------------------------|\n");
        return sb.toString();
    }

    public String connectionsDomain() {
        StringBuilder sb = new StringBuilder();
        sb.append("|-----------------------------------------------|----------------|\n");
        sb.append("|                  Connections                  |      PING      |\n");
        sb.append("|-----------------------------------------------|----------------|\n");
        for (int v = 0; v < graph.V(); v++) {
            for (Edge e : graph.adj(v)) {
                String ipString = String.format("%s -> %s", getDomainByIP(IPs.get(v)), getDomainByIP(IPs.get(e.other(v))));
                String time = String.format("(PING: %d ms.)", (int) e.weight());
                sb.append(String.format("| %-45s | %5s |\n", ipString, time));
            }
            sb.append("|-----------------------------------------------|----------------|\n");
        }
        return sb.toString();
    }

    public String connectionsIP() {
        StringBuilder sb = new StringBuilder();
        sb.append("|-----------------------------------------------|----------------|\n");
        sb.append("|                  Connections                  |      PING      |\n");
        sb.append("|-----------------------------------------------|----------------|\n");
        for (int v = 0; v < graph.V(); v++) {
            for (Edge e : graph.adj(v)) {
                String ipString = String.format("%s -> %s", IPs.get(v), IPs.get(e.other(v)));
                String time = String.format("(PING: %d ms.)", (int) e.weight());
                sb.append(String.format("| %-45s | %5s |\n", ipString, time));
            }
            sb.append("|-----------------------------------------------|----------------|\n");
        }
        return sb.toString();
    }

    public String getIPbyDomain(String domain) {
        boolean contains = false;
        String IP = "";
        for (String ip : st.keys()) {
            for (String s : st.get(ip)) {
                if (s.equals(domain)) {
                    IP = ip;
                    contains = true;
                    break;
                }
            }
            if (contains) break;
        }
        if (!contains) throw new NoSuchElementException("this domain isn't in domain list");
        else return IP;
    }

    public String getDomainByIP(String IP) {
        if (!st.contains(IP)) throw new NoSuchElementException("this IP isn't in IP list");
        return st.get(IP).peek();
    }

    public void deleteEdge(String firstIP, String secondIP) {
        if (IPs.contains(firstIP) && IPs.contains(secondIP)) {
            graph.deleteEdge(IPs.indexOf(firstIP), IPs.indexOf(secondIP));
        } else {
            throw new IllegalArgumentException("Error: list doesn't contains one or two IPs");
        }
    }

    public void bfs(String firstDomain, String secondDomain) {
        String firstIP = getIPbyDomain(firstDomain);
        String secondIP = getIPbyDomain(secondDomain);
        System.out.println("\nBreadth-First Search:");
        int s = IPs.indexOf(firstIP);
        int v = IPs.indexOf(secondIP);
        BreadthFirstPaths bfs = new BreadthFirstPaths(graph, s);
        if (bfs.hasPathTo(v)) {
            System.out.printf("%s to %s (%d edges):\n", firstDomain, secondDomain, bfs.distTo(v));
            for (int x : bfs.pathTo(v)) {
                if (x == s) System.out.print(getDomainByIP(IPs.get(x)));
                else System.out.print(" -> " + getDomainByIP(IPs.get(x)));
            }
            System.out.println();
        } else {
            System.out.printf("%s to %s (-):  not connected\n", firstDomain, secondDomain);
        }
    }

    public void dfs(String firstDomain, String secondDomain) {
        String firstIP = getIPbyDomain(firstDomain);
        String secondIP = getIPbyDomain(secondDomain);
        System.out.println("\nDepth-First Search:");
        int s = IPs.indexOf(firstIP);
        int v = IPs.indexOf(secondIP);
        DepthFirstPaths dfs = new DepthFirstPaths(graph, s);
        if (dfs.hasPathTo(v)) {
            System.out.printf("%s to %s:\n", firstDomain, secondDomain);
            for (int x : dfs.pathTo(v)) {
                if (x == s) System.out.print(getDomainByIP(IPs.get(x)));
                else System.out.print(" -> " + getDomainByIP(IPs.get(x)));
            }
            System.out.println();
        } else {
            System.out.printf("%s to %s (-):  not connected\n", firstDomain, secondDomain);
        }
    }

    public void weightedSP(String firstDomain, String secondDomain) {
        String firstIP = getIPbyDomain(firstDomain);
        String secondIP = getIPbyDomain(secondDomain);
        System.out.println("\nDijkstra Algorithm.\nShortest path:");
        int s = IPs.indexOf(firstIP);
        int v = IPs.indexOf(secondIP);
        DijkstraSP sp = new DijkstraSP(graph, s);
        if (sp.hasPathTo(v)) {
            System.out.printf("%s to %s (%dms.): \n", firstDomain, secondDomain, (int) sp.distTo(v));
            for (Edge e : sp.pathTo(v)) {
                String one = getDomainByIP(IPs.get(e.either()));
                String two = getDomainByIP(IPs.get(e.other(e.either())));
                System.out.printf("%s - %s (%dms.)\t\t", one, two, (int) e.weight());
            }
            System.out.println();
        } else {
            System.out.printf("%s to %s (-):  not connected\n", firstDomain, secondDomain);
        }
    }
}