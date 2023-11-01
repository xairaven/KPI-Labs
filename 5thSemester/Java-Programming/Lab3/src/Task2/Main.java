package Lab3.Task2;

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        var scanner = new Scanner(System.in);

        System.out.print("Введіть пароль: ");
        var password = scanner.nextLine().trim().toLowerCase();
        Validator.check(password);
    }
}
