package Lab1.sort;

/**
 * Метод сортування вставками
 */
public class Insertion {
    private static int iterations = 0;

    public static void sort(int[] arr) {
        for (int i = 1; i < arr.length; i++) {
            for (int j = i; j > 0 && arr[j] < arr[j - 1]; j--) {
                exch(arr, j, j - 1);
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
