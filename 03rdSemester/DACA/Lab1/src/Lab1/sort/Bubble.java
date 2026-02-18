package Lab1.sort;

/**
 * Метод сортування бульбашкою
 */
public class Bubble {
    private static int iterations = 0;

    public static void sort(int[] arr) {
        boolean sorted = false;
        while (!sorted) {
            sorted = true;
            for (int i = 0; i < arr.length - 1; i++) {
                if (arr[i + 1] < arr[i]) {
                    exch(arr, i, i + 1);
                    sorted = false;
                }
            }
        }
    }

    private static void exch(int[] a, int i, int j) {
        int t = a[i];
        a[i] = a[j];
        a[j] = t;

        iterations++;
    }

    public static int getIterations() {
        return iterations;
    }
}
