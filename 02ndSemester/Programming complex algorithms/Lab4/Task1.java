package Lab4;
import java.util.Scanner;

public class Task1 {
    public static void main(int[][] matrix) {
        Scanner scan = new Scanner(System.in);

        System.out.println("\n-- Завдання 1 --\nМетод пошуку числа з бар'єром");
        System.out.println("Введіть число від 1 до 50");
        int key = scan.nextInt();
        System.out.printf("Пошук числа %d:\n", key);

        boolean found = false;
        for (int i = 0; i < matrix.length; i++) {
            int j = Search.barrier(key, matrix[i]);
            if(j != -1) {
                System.out.printf("Рядок %d; Стовпець %d\n", i + 1, j + 1);
                found = true;
            }
        }
        if (!found) {
            System.out.println("Елемент не знайдено");
        }
    }
}
