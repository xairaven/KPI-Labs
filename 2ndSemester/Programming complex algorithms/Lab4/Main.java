package Lab4;

public class Main {
    public static void main(String[] args) {
        final int SIZE = 10;

        // generate and show matrix
        int[][] matrix = genMatrix(SIZE);
        System.out.println("\n*Невпорядкована матриця*");
        showMatrix(matrix);

        // task 1
        //Task1.main(matrix);

        // task 2
        Task2.main(matrix);
    }

    private static int[][] genMatrix(int size) {
        final int MIN = 0;
        final int MAX = 50;
        int[][] arr = new int[size][size];
        for (int i = 0; i < size; i++) {
            for (int j = 0; j < size; j++) {
                arr[i][j] = (int) (Math.random() * (MAX - MIN + 1) + MIN);
            }
        }
        return arr;
    }

    static void showMatrix(int[][] arr) {
        for (int[] a : arr) {
            for (int i : a) {
                System.out.printf("%d\t", i);
            }
            System.out.println();
        }
    }
}
