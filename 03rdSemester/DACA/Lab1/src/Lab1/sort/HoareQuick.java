package Lab1.sort;

/**
 * Швидке сортування Гоара
 */
public class HoareQuick {
    private static int iterations = 0;

    //створюємо метод для швидкого сортування
    private static void sort(int[] a, int lo, int hi) {
        int mid = lo + (hi - lo) / 2;
        double pivot = a[mid];
        int i = lo, j = hi;
        while (i <= j) {
            while (a[i] < pivot) {
                i++;
            }

            while (a[j] > pivot) {
                j--;
            }

            if (i <= j) {
                exch(a, i, j);
                i++;
                j--;
            }
        }

        if (lo < j) {
            sort(a, lo, j);
        }
        if (hi > i) {
            sort(a, i, hi);
        }
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
