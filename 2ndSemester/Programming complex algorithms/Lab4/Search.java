package Lab4;

public class Search {
    // binary search start
    public static int binary(int key, int[] arr) {
        assert isSorted(arr);
        return binary(key, arr, 0, arr.length - 1);
    }

    // binary search
    private static int binary(int key, int[] arr, int lo, int hi) {
        if (arr == null) throw new IllegalArgumentException("Exception: arr = null");
        if (lo <= hi) {
            int middle = lo + (hi - lo)/2;
            if (key < arr[middle]) return binary(key, arr, lo, middle - 1);
            else if (key > arr[middle]) return binary(key, arr, middle + 1, hi);
            else return middle;
        } else return -1;
    }

    // linear search
    public static int linear(int key, int[] arr) {
        if (arr == null) throw new IllegalArgumentException("Exception: arr = null");
        for (int i = 0; i < arr.length; i++) {
            if (arr[i] == key) return i+1;
        }
        return -1;
    }

    // search with barrier
    public static int barrier(int key, int[] arr) {
        if (arr == null) throw new IllegalArgumentException("Exception: arr = null");
        final int SIZE = arr.length;
        int last = arr[SIZE - 1];
        arr[SIZE - 1] = key;
        int res;
        for (res = 0; arr[res] != key; res++);
        arr[SIZE - 1] = last;
        if (res != SIZE - 1 || key == last) return res;
        return -1;
    }

    // search a subsequence
    public static int subsequence(int[] pattern, int[] arr) {
        if (arr == null || pattern == null) throw new IllegalArgumentException("Exception: arr or pattern = null");
        if (pattern.length > arr.length) return -1;
        for (int i = 0; i + (pattern.length - 1) < arr.length; i++) {
            if (arr[i] == pattern[0]) {
                boolean subSeq = true;
                for (int j = i + 1, k = 1; j < arr.length && k < pattern.length; j++, k++) {
                    if (arr[j] != pattern[k]) {
                        subSeq = false;
                        break;
                    }
                }
                if (subSeq) return i;
            }
        }
        return -1;
    }

    // Rabin-Karp algorithm for searching pattern in a subsequence
    public static int RabinKarp(int[] patternArray, int[] arr) {
        // checking NullPointer
        if (arr == null || patternArray == null) throw new IllegalArgumentException("Exception: arr or pattern = null");
        // pattern longer than array?
        if (patternArray.length > arr.length) return -1;

        // prime number & number of "letters"
        final int prime = 13;
        final int alphabet = 25;

        // building SB objects
        StringBuilder patternSB = new StringBuilder();
        StringBuilder arraySB = new StringBuilder();
        for (int a: patternArray) {
            patternSB.append(a);
        }
        for (int b: arr) {
            arraySB.append(b);
        }

        // getting strings of pattern and array
        String pattern = patternSB.toString();
        String txt = arraySB.toString();

        int j;
        int patLen = pattern.length();
        int txtLen = txt.length();

        int patternHash = 0, txtHash = 0, hash = 1;

        for (int i = 0; i < patLen - 1; i++) {
            hash = (hash * alphabet) % prime;
        }

        for (int i = 0; i < patLen; i++) {
            txtHash = (alphabet * txtHash + txt.charAt(i)) % prime;
            patternHash = (alphabet * patternHash + pattern.charAt(i)) % prime;
        }

        for (int i = 0; i <= txtLen - patLen; i++) {
            if (patternHash == txtHash) {
                for (j = 0; j < patLen; j++) {
                    if (txt.charAt(i + j) != pattern.charAt(j)) break;
                }

                if (j == patLen) return i;
            }

            if (i < txtLen - patLen) {
                txtHash = (alphabet * (txtHash - txt.charAt(i) * hash) + txt.charAt(i + patLen)) % prime;
                if (txtHash < 0) {
                    txtHash += prime;
                }
            }
        }
        return -1;
    }

    // is array sorted
    private static boolean isSorted(int[] a) {
        for (int i = 1; i < a.length; i++) {
            if (a[i] < (a[i-1])) return false;
        }
        return true;
    }
}
