package Lab1;

@SuppressWarnings("unchecked")
public class SortSelection {
    public static void sort(Comparable[] arr) {
        for (int i = 0; i < arr.length; i++) {
            int minId = i;
            for (int j = i + 1; j < arr.length; j++) {
                if (less(arr[j], arr[minId])) {
                    minId = j;
                }
            }
            exch(arr, i, minId);
        }
    }

    private static boolean less(Comparable v, Comparable w) {
        return v.compareTo(w) < 0;
    }

    private static void exch(Comparable[] a, int i, int j) {
        Comparable t = a[i];
        a[i] = a[j];
        a[j] = t;
    }

    public static boolean isSorted(Comparable[] arr) {
        for (int i = 1; i < arr.length; i++) {
            if (arr[i].compareTo(arr[i - 1]) < 0) return false;
        }
        return true;
    }
}
