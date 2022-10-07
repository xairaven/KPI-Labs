package Lab1.sort;

/**
 * Метод сортування Шелла
 */
public class Shell {
    private static int iterations = 0;

    public static void sort(int[] arr) {
        int N = arr.length;
        int h = 1;
        while (h < N/3) h = 3*h+1;
        while (h > 0) {
            for (int i = h; i < N; i++) {
                for (int j = i; j >= h && arr[j] < arr[j - h]; j -= h) {
                    exch(arr, j, j - h);
                }
            }
            h /= 3;
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
