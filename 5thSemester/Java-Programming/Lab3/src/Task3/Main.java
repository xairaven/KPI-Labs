package Lab3.Task3;

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        var scanner = new Scanner(System.in);

        System.out.println("Введіть текстове повідомлення:");
        var message = scanner.nextLine();

        var result = SecureText.mask(message);

        System.out.printf("Результат: \n%s", result);
    }
}