package Lab2.Task3;

import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        var scanner = new Scanner(System.in);

        // Prices for basic service packages
        final var priceEconomy = 20.0;
        final var priceStandard = 50.0;
        final var pricePremium = 80.0;

        // Additional options and their prices
        final var extraGigsPrice = 5.0;
        final var extraMinutesPrice = 0.1;
        final var extraMinutesOtherOpsPrice = 0.2;

        // Entering the type of service and duration of service provision (in months)
        System.out.print("Enter the type of service (Budget, Standard, Premium): ");
        var serviceType = scanner.nextLine();

        System.out.print("Enter the duration of service provision (in months): ");
        var months = scanner.nextInt();
        if (months < 0) throw new IllegalArgumentException("Number must be non-negative.");

        // Calculation of the total cost of the service
        double totalCost = 0.0;

        if (serviceType.equalsIgnoreCase("Budget")) {
            totalCost += priceEconomy * months;
        } else if (serviceType.equalsIgnoreCase("Standard")) {
            totalCost += priceStandard * months;
        } else if (serviceType.equalsIgnoreCase("Premium")) {
            totalCost += pricePremium * months;
        } else {
            System.out.println("Incorrectly entered service type.");
            return;
        }

        // Entering additional options
        System.out.print("Enter the number of additional gigabytes: ");
        int extraGigs = scanner.nextInt();
        if (extraGigs < 0) throw new IllegalArgumentException("Number must be non-negative.");
        totalCost += extraGigs * extraGigsPrice;

        System.out.print("Enter the number of additional minutes for calls: ");
        int extraMinutes = scanner.nextInt();
        if (extraMinutes < 0) throw new IllegalArgumentException("Number must be non-negative.");
        totalCost += extraMinutes * extraMinutesPrice;

        System.out.print("Enter the number of additional minutes for calls to other operators: ");
        int extraMinutesOtherOps = scanner.nextInt();
        if (extraMinutesOtherOps < 0) throw new IllegalArgumentException("Number must be non-negative.");
        totalCost += extraMinutesOtherOps * extraMinutesOtherOpsPrice;

        // Result
        System.out.printf("The total cost of the service: %.1f UAH", totalCost);
    }
}
