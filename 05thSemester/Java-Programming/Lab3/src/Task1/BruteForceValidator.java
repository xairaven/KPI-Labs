package Lab3.Task1;

public class BruteForceValidator {
    public static void cardinality(int c) {
        if (c <= 0) throw new IllegalArgumentException("Cardinal must be greater than 0");
    }

    public static void passwordLength(int l) {
        if (l <= 0) throw new IllegalArgumentException("Password length must be greater than 0");
    }

    public static void speed(int s) {
        if (s <= 0) throw new IllegalArgumentException("Password picking speed must be greater than 0");
    }
}
