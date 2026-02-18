package Lab3.Task1;

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        int A = 26;
        int L = 12;
        int V = 5;

        var scanner = new Scanner(System.in);

        System.out.println("Провести розрахунки з параметрами за замовчуванням?");
        System.out.printf("(A = %d, L = %d, V = %d)\n", A, L, V);
        System.out.print("(Y/n): ");
        if (!scanner.next().toLowerCase().trim().equals("y")) {
            System.out.print("1. Кардинал алфавіту: ");
            A = scanner.nextInt();

            System.out.print("2. Довжина паролю: ");
            L = scanner.nextInt();

            System.out.print("3. Швидкість підбору (в сек.): ");
            V = scanner.nextInt();
        }

        var calculator = new BruteForceCalculator(A, L, V);
        var seconds = calculator.seconds();
        System.out.println(TimeFormatter.seconds(seconds));
    }
}