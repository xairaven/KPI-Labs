package Lab2.Task1;

import java.time.Month;
import java.time.YearMonth;
import java.util.InputMismatchException;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        var scanner = new Scanner(System.in);

        System.out.print("Enter a number from 1 to 12 (month): ");

        int inputMonth;
        // Check if the user input is valid (between 1 and 12)
        try {
            inputMonth = scanner.nextInt();
        } catch (InputMismatchException e) {
            System.out.println("Expected number. Try again, please!");
            return;
        }

        if (inputMonth < 1 || inputMonth > 12) {
            System.out.println("Invalid month. Please enter a month between 1 and 12.");
            return;
        } else {
            System.out.println("Input passed verification!");
        }

        // Get the Month enum from the user input
        var month = Month.of(inputMonth);

        // Get the number of days in the month
        var yearMonth = YearMonth.of(YearMonth.now().getYear(), month);
        int daysInMonth = yearMonth.lengthOfMonth();

        System.out.println("Number of days in the month: " + daysInMonth);
    }
}