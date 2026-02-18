package Lab1.sort;

/**
 * Метод сортування вибором
 */
public class Selection {
    private static int iterations = 0;

    public static void sort(int[] arr) {
        for (int i = 0; i < arr.length; i++) {
            int min = i;
            for (int j = i + 1; j < arr.length; j++) {
                if (arr[j] < arr[min]) {
                    min = j;
                }
            }
            exch(arr, i, min);
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
