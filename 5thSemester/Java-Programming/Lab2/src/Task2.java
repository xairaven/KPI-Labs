package Lab2.Task2;

import java.util.InputMismatchException;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        var scanner = new Scanner(System.in);

        System.out.print("Enter a number from 1 to 7 (weekday): ");

        String weekday;
        try {
            int inputWeekday = scanner.nextInt();
            weekday = getWeekdayText(inputWeekday);
        } catch (IllegalArgumentException e) {
            System.out.println(e.getMessage());
            return;
        } catch (InputMismatchException e) {
            System.out.println("Expected number. Try again, please!");
            return;
        }

        System.out.println("Result: " + weekday);
    }

    private static String getWeekdayText(int number) {
        if (number < 1 || number > 7) {
            throw new IllegalArgumentException("Invalid weekday. Expected number between 1 and 7.");
        }

        return switch (number) {
            case 1 -> "Monday";
            case 2 -> "Tuesday";
            case 3 -> "Wednesday";
            case 4 -> "Thursday";
            case 5 -> "Friday";
            case 6 -> "Saturday";
            case 7 -> "Sunday";
            default -> throw new IllegalArgumentException("Unexpected value: " + number);
        };
    }
}
