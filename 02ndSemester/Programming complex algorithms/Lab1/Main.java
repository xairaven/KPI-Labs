package Lab1;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner scan = new Scanner(System.in);
        System.out.println("-- Lab1 v.21 --");
        System.out.println("Menu:\n 1. Task 1\n 2. Task 2\n 3. Testing task 1\n 4. Testing task 2");
        System.out.println("Enter the number of the task you want to see (1-3): ");
        int task = scan.nextInt();
        switch (task) {
            case 1 -> Task1.main();
            case 2 -> Task2.main();
            case 3 -> TestingTask1.main();
            case 4 -> TestingTask2.main();
            default -> throw new IllegalArgumentException("Wrong number. Please, restart program and input number");
        }
    }
}
