package Lab1;

import java.util.Scanner;
import Lab1.misc.*;
import Lab1.sort.*;

/**
 * Лабораторна робота №1
 * Завдання: Ознайомитись з алгоритмами сортування масивів та способами їхньої реалізації.
 * У якості індивідуального завдання необхідно написати програмний код, у
 * якому реалізується сортування масивів методами бульбашки, вставок, вибору,
 * сортуванням Шелла, Гоара та швидкого сортування.
 * Виконати порівняння ефективності вказаних методів сортування.
 * Індивідуальне завдання:
 * Задана матриця N*N. Відсортувати всі стовпці за зростанням.
 *
 * Виконав студент групи ТР-12
 * Ковальов Олександр
 * Номер варіанту: 22
*/
public class Main {
    public static void main(String[] args) {
        Scanner scan = new Scanner(System.in);
        System.out.println("Введіть розмірність матриці: ");
        int N = scan.nextInt();

        int[][] initMatrix = Matrix.gen(N);

        System.out.println("\nПочаткова матриця: ");
        Matrix.show(initMatrix);
        System.out.println("\n");

        /*-------------------------------------------------------*/

        Timer timer = new Timer();
        int[][] tempMatrix = Matrix.sort(Bubble::sort, initMatrix, timer);

        System.out.println("Результат, сортування бульбашкою: ");
        Matrix.show(tempMatrix);
        System.out.printf("Час: %d наносекунд\n", timer.getTime());
        System.out.println("Кількість ітерацій: " + Bubble.getIterations());
        System.out.println("\n");

        /*-------------------------------------------------------*/

        timer = new Timer();
        tempMatrix = Matrix.sort(Selection::sort, initMatrix, timer);

        System.out.println("Результат, сортування вибором: ");
        Matrix.show(tempMatrix);
        System.out.printf("Час: %d наносекунд\n", timer.getTime());
        System.out.println("Кількість ітерацій: " + Selection.getIterations());
        System.out.println("\n");

        /*-------------------------------------------------------*/

        timer = new Timer();
        tempMatrix = Matrix.sort(Insertion::sort, initMatrix, timer);

        System.out.println("Результат, сортування вставками: ");
        Matrix.show(tempMatrix);
        System.out.printf("Час: %d наносекунд\n", timer.getTime());
        System.out.println("Кількість ітерацій: " + Insertion.getIterations());
        System.out.println("\n");

        /*-------------------------------------------------------*/

        timer = new Timer();
        tempMatrix = Matrix.sort(Shell::sort, initMatrix, timer);

        System.out.println("Результат, сортування Шелла: ");
        Matrix.show(tempMatrix);
        System.out.printf("Час: %d наносекунд\n", timer.getTime());
        System.out.println("Кількість ітерацій: " + Shell.getIterations());
        System.out.println("\n");

        /*-------------------------------------------------------*/

        timer = new Timer();
        tempMatrix = Matrix.sort(Quick::sort, initMatrix, timer);

        System.out.println("Результат, швидке сортування: ");
        Matrix.show(tempMatrix);
        System.out.printf("Час: %d наносекунд\n", timer.getTime());
        System.out.println("Кількість ітерацій: " + Quick.getIterations());
        System.out.println("\n");

        /*-------------------------------------------------------*/

        timer = new Timer();
        tempMatrix = Matrix.sort(HoareQuick::sort, initMatrix, timer);

        System.out.println("Результат, швидке сортування Гоара: ");
        Matrix.show(tempMatrix);
        System.out.printf("Час: %d наносекунд\n", timer.getTime());
        System.out.println("Кількість ітерацій: " + HoareQuick.getIterations());
    }
}
