package Lab1;
import java.util.Scanner;

public class TestingTask1 {
    public static void main() {
        Scanner scan = new Scanner(System.in);
        System.out.println("\nNumber of elements in array:");
        int elems = scan.nextInt();
        System.out.println("\nHow many times:");
        int times = scan.nextInt();
        System.out.printf(" * Generating array: %.4f ms.\n",testGen(elems, times));
        System.out.printf(" * Algorithm: %.4f ms.\n", testAlg(elems, times));
    }

    public static double testGen(int elems, int times) {
        long startTime = System.currentTimeMillis();
        for (int i = 0; i < times; i++) {
            int[][] arr = Task1.genQuadArray(elems);
        }
        return ((double) (System.currentTimeMillis() - startTime)) / ((double) times);
    }

    public static double testAlg(int elems, int times) {
        int[][] origArray = Task1.genQuadArray(elems);
        long sum = 0;
        for (int i = 0; i < times; i++) {
            int[][] arr1 = Task1.copy(origArray);
            long startTime = System.currentTimeMillis();
            Task1.evenToStartBubble(arr1);
            sum += (System.currentTimeMillis() - startTime);
        }
        return sum;
    }
}
