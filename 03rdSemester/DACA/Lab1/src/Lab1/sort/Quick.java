package Lab1.sort;

/**
 * Швидке сортування
 */
public class Quick {
    private static int iterations = 0;

    private static int partition(int[] a, int lo, int hi) {
        int i = lo, j = hi + 1;
        while (true) {
            while (a[++i] < a[lo]) {
                if (i == hi) break;
            }

            while (a[lo] < a[--j]) {
                if (j == lo) break;
            }

            if (i >= j) break;
            exch(a, i, j);
        }
        exch(a, lo, j);
        return j;
    }

    private static void sort(int[] a, int lo, int hi) {
        if (hi <= lo) return;
        int j = partition(a, lo, hi);
        sort(a, lo, j-1);
        sort(a, j+1, hi);
    }

    public static void sort(int[] a) {
        sort(a, 0, a.length - 1);
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
