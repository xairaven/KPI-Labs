package Lab3;

public class Main {
    public static void main(String[] args) {
        // checking args
        if (args.length < 2) {
            System.out.println("Введіть розмірність масиву та визначіться, випадковий масив чи ні");
            System.out.println("Приклад: 10 true (створює масив випадкових чисел 10х10");
            return;
        }

        // parsing values from console
        final int SIZE = Integer.parseInt(args[0]);
        final boolean RANDOM = Boolean.parseBoolean(args[1]);

        // generate matrix
        int[][] matrix = genMatrix(SIZE, RANDOM);

        // show start matrix
        System.out.println("*Початкова матриця*");
        showMatrix(matrix);

        // bubble-sort
        sort(matrix);

        // show sorted matrix
        System.out.println("\n\n*Відсортована матриця*");
        showMatrix(matrix);
    }

    private static int[][] genMatrix(int size, boolean random) {
        int[][] arr = new int[size][size];
        if (random) {
            for (int i = 0; i < size; i++) {
                for (int j = 0; j < size; j++) {
                    arr[i][j] = getRandomNum(0, 100);
                }
            }
        } else {
            for (int i = 0; i < size; i++) {
                for (int j = 0; j < size; j++) {
                    arr[i][j] = (size - i)*10 - j;
                }
            }
        }

        return arr;
    }

    private static int getRandomNum(int min, int max) {
        return (int) (Math.random() * (max - min + 1) + min);
    }

    private static void showMatrix(int[][] arr) {
        for (int[] a : arr) {
            for (int i : a) {
                System.out.printf("%d\t", i);
            }
            System.out.println();
        }
    }

    private static void sort(int[][] arr) {
        final int SIZE = arr.length;
        for (int j = 0; j < SIZE; j++) {
            boolean sorted = false;
            while (!sorted) {
                sorted = true;
                // i < j && j < (SIZE - i - 1) - Верхній трикутник матриці.
                for (int i = 0; i < j && j < (SIZE - i - 1); i++) {
                    if (arr[i][j] > arr[i+1][j]) {
                        // обмін значень
                        int temp = arr[i][j];
                        arr[i][j] = arr[i+1][j];
                        arr[i+1][j] = temp;
                        sorted = false;
                    }
                }
            }
        }
    }
}
