package Lab2;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        System.out.println("-- Lab 2 --");

        System.out.println("Input N:");
        Scanner scan = new Scanner(System.in);
        int n = scan.nextInt();
        if (n < 1) throw new IllegalArgumentException("N must be greater than 1");

        System.out.println("Loop based algorithm");
        long start = System.nanoTime();
        System.out.printf("Result: %.20f\n", loop(n));
        long time = System.nanoTime() - start;
        System.out.printf("Time: %d ns.", time);
        
        System.out.println("\n\nRecursive based algorithm");
        start = System.nanoTime();
        try {
            System.out.printf("Result: %.20f\n", recursive(n));
        } catch (StackOverflowError e) {
            System.out.println("Stack overflow exception");
        }
        time = System.nanoTime() - start;
        System.out.printf("Time: %d ns.", time);
    }

    public static double recursive(int n) {
        if (n < 1) return 1;
        return ((2 * n + 1) / Math.pow(n, 3)) * recursive(n - 1);
    }

    public static double loop(int n) {
        double result = 1;
        for (int i = 1; i <= n; i++) {
            result *= (2 * i + 1) / Math.pow(i, 3);
        }
        return result;
    }
}
