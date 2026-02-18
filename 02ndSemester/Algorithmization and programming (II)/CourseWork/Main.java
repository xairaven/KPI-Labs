package ua.kpi;
import java.util.NoSuchElementException;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        final String ip = "src/ua/kpi/resources/ip.csv";
        final String connections = "src/ua/kpi/resources/connections.txt";
        IPGraph ipGraph = new IPGraph(ip, connections);
        Scanner scan = new Scanner(System.in);
        boolean EXIT = false;

        while(!EXIT) {
            System.out.println("""
                    |----------------------------------------------|
                    |                     MENU:                    |
                    |----------------------------------------------|
                    | 1. Print domains/IP:                         |
                    | 2. Print connections (Domains):              |
                    | 3. Print connections (IPs):                  |
                    | 4. IP of domain                              |
                    | 5. Domain of IP                              |
                    | 6. Delete edge                               |
                    | 7. BFS                                       |
                    | 8. DFS                                       |
                    | 9. Dijkstra algorithm                        |
                    | 10. EXIT                                     |
                    |----------------------------------------------|
                    """);
            switch (scan.nextInt()) {
                case 1 -> printDomains(ipGraph);
                case 2 -> printDomainConnections(ipGraph);
                case 3 -> printIPConnections(ipGraph);
                case 4 -> ipOfDomain(ipGraph);
                case 5 -> domainOfIP(ipGraph);
                case 6 -> deleteEdge(ipGraph);
                case 7 -> BFS(ipGraph);
                case 8 -> DFS(ipGraph);
                case 9 -> Dijkstra(ipGraph);
                case 10 -> EXIT = true;
                default -> System.out.println("Wrong choice. Try again");
            }
        }
    }

    private static void printDomains(IPGraph ipGraph) {
        System.out.println(ipGraph.domainIPList());
    }

    private static void printDomainConnections(IPGraph ipGraph) {
        System.out.println(ipGraph.connectionsDomain());
    }

    private static void printIPConnections(IPGraph ipGraph) {
        System.out.println(ipGraph.connectionsIP());
    }

    private static void ipOfDomain(IPGraph ipGraph) {
        Scanner scanDomain = new Scanner(System.in);
        System.out.println("Domain:");
        try {
            System.out.println("IP: " + ipGraph.getIPbyDomain(scanDomain.nextLine()));
        } catch (NoSuchElementException e) {
            System.out.println("This domain isn't in domain list");
        }
    }

    private static void domainOfIP(IPGraph ipGraph) {
        Scanner scanIP = new Scanner(System.in);
        System.out.println("IP:");
        try {
            System.out.println("Domain: " + ipGraph.getDomainByIP(scanIP.nextLine()));
        } catch (NoSuchElementException e) {
            System.out.println("This IP isn't in IP list");
        }
    }

    private static void deleteEdge(IPGraph ipGraph) {
        Scanner scanIP = new Scanner(System.in);
        System.out.println("You want to break connection. Type first IP");
        String firstIP = scanIP.nextLine();
        System.out.println("Second IP:");
        String secondIP = scanIP.nextLine();
        try {
            ipGraph.deleteEdge(firstIP, secondIP);
            System.out.println("Done!");
        } catch (IllegalArgumentException e) {
            System.out.println("Error: list doesn't contains one or two IPs");
        } catch (NoSuchElementException e) {
            System.out.println("Error: this IPs don't have connection");
        }
    }

    private static void BFS(IPGraph ipGraph) {
        Scanner scanVertex = new Scanner(System.in);
        System.out.println("Type domain, from which you want to see BFS");
        String first = scanVertex.nextLine();
        System.out.println("Type domain, to which you want to see BFS");
        String second = scanVertex.nextLine();
        try {
            ipGraph.bfs(first, second);
        } catch (NoSuchElementException e) {
            System.out.println("Error: one or two of this IPs isn't in list");
        }
    }

    private static void DFS(IPGraph ipGraph) {
        Scanner scanVertex = new Scanner(System.in);
        System.out.println("Type domain, from which you want to see DFS");
        String first = scanVertex.nextLine();
        System.out.println("Type domain, to which you want to see DFS");
        String second = scanVertex.nextLine();
        try {
            ipGraph.dfs(first, second);
        } catch (NoSuchElementException e) {
            System.out.println("Error: one or two of this IPs isn't in list");
        }
    }

    private static void Dijkstra(IPGraph ipGraph) {
        Scanner scanVertex = new Scanner(System.in);
        System.out.println("Type domain, from which you want to see shortest path");
        String first = scanVertex.nextLine();
        System.out.println("Type domain, to which you want to see shortest path");
        String second = scanVertex.nextLine();
        try {
            ipGraph.weightedSP(first, second);
        } catch (NoSuchElementException e) {
            System.out.println("Error: one or two of this IPs isn't in list");
        }
    }
}
