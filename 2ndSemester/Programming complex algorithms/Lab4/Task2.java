package Lab4;
import java.util.Arrays;
import java.util.Scanner;

public class Task2 {
    public static void main(int[][] matrix) {
        Scanner scan = new Scanner(System.in);

        // choosing method
        System.out.println("\n-- Завдання 2 --");
        System.out.println("Оберіть метод:\n " +
                "1. Бінарний пошук\n " +
                "2. Пошук послідовності елементів в масиві\n " +
                "3. Алгоритм Рабіна-Карпа");
        int choice = scan.nextInt();

        // sorting matrix
        System.out.println("\n*Відсортована матриця*");
        for (int[] arr: matrix) {
            Arrays.sort(arr);
        }
        Main.showMatrix(matrix);

        // searching & scanning keys and patterns
        boolean found = false;
        switch (choice) {
            case 1 -> {
                //scanning
                System.out.println("Введіть число від 1 до 50");
                int key = scan.nextInt();
                System.out.printf("Пошук числа %d:\n", key);

                //searching
                for (int i = 0; i < matrix.length; i++) {
                    int j = Search.binary(key, matrix[i]);
                    if(j != -1) {
                        System.out.printf("Рядок %d; Стовпець %d\n", i + 1, j + 1);
                        found = true;
                    }
                }
            } case 2 -> {
                //scanning
                System.out.println("З скількох чисел складається послідовність?");
                int amount = scan.nextInt();
                System.out.println("Введіть послідовність");
                int[] pattern = new int[amount];
                for (int i = 0; i < amount; i++) {
                    pattern[i] = scan.nextInt();
                }

                //searching
                for (int i = 0; i < matrix.length; i++) {
                    int j = Search.subsequence(pattern, matrix[i]);
                    if(j != -1) {
                        System.out.printf("Рядок %d; Стовпець %d\n", i + 1, j + 1);
                        found = true;
                    }
                }
            } case 3 -> {
                //scanning
                System.out.println("З скількох чисел складається послідовність?");
                int amount = scan.nextInt();
                System.out.println("Введіть послідовність");
                int[] pattern = new int[amount];
                for (int i = 0; i < amount; i++) {
                    pattern[i] = scan.nextInt();
                }

                //searching
                for (int i = 0; i < matrix.length; i++) {
                    int j = Search.RabinKarp(pattern, matrix[i]);
                    if(j != -1) {
                        System.out.printf("Рядок %d; Стовпець %d\n", i + 1, j + 1);
                        found = true;
                    }
                }
            } default -> {
                System.out.println("Метод не знайдено");
                return;
            }
        }
        if (!found) {
            System.out.println("Елемент не знайдено");
        }
    }
}
