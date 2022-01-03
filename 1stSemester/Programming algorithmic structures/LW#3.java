import java.util.Scanner;
public class LR3 {
    public static void main(String[] args) {
        System.out.println("Лабораторна робота №3, в.23 (Ковальов о. ТР-12)\n");
        System.out.println("--- Task 1 ---\n");
        task1(-1, 1, 0.05); //min = -1; max = 1; h = 0.05;
        
        System.out.println("\n--- Task 2 ---\n");
        System.out.println("Послідовність 1:"); //Вводимо послідовність
        double[] arr1 = {10.1, -2.5, 3, 86, 18.7, 5.95, 11.2, 8.956, 1.23, -12.3, 12.4, -9.6};
        for (int i = 0; i < arr1.length; i++) { //Виводимо її
            System.out.printf("%.3f\t\t", arr1[i]);
        }
        double result = task2(arr1); //Отримуємо результат
        System.out.printf("\nРезультат: %.4f\n", task2(arr1));
        System.out.println("\nПослідовність 2:");
        double[] arr2 = {1, 2, 3, 4, -60, 45, 1.1, 1, 2, 8.7, 0.95};
        for (int i = 0; i < arr2.length; i++) {
            System.out.printf("%.3f\t\t", arr2[i]);
        }
        System.out.printf("\nРезультат: %.4f\n", task2(arr2));

        System.out.println("\n--- Task 3 ---\n(Введіть 0.6 та 2.8, х1 та х2 відповідно)\n");
        Scanner scan = new Scanner(System.in); //Створюємо сканер
        double x = 0;
        for (int i = 0; i < 2; i++) { //Цикл для того, щоб повторити дію двічі
            x = scan.nextDouble();
            System.out.printf("Сумма ряду з x%d = %.1f: %.4f\n", i + 1, x, task3(x)); //Виводимо результат
        }
    }

    public static void task1(int min, int max, double h) {
        double y = 0;
        double z = 0;
        double x = min;
        System.out.printf("y = f(x), cos(1/(x + Pi/3))\nz = f(x), e^(3(x - 0.6))\n");
        System.out.printf("\nІнтервал: [%d; %d]\nКрок приросту аргументу: %.2f\n", min, max, h);
        System.out.printf("\n  x\t\t\t  y(x)\t\t z(x)\n");
        while (x <= (max + 0.01)) { //Приклад циклу while
            y = Math.cos(1 / (x + Math.PI/3));
            z = Math.exp(3 * (x - 0.6));
            System.out.printf("%.2f\t\t%.4f\t\t%.4f\n", x, y, z);
            x += h;
        }
    }

    public static double task2(double[] arr) {
        double sum = 0;
        for (int i = 1; i < arr.length; i++) {
            if (arr[i] < arr[0]) sum += Math.pow(arr[i], 2);
        }
        return sum;
    }

    public static double task3(double x) {
        int k = 1;
        double sum = 0;
        do {
            sum += (Math.pow(-1, k) * factorial(k))/(Math.sqrt(x) + Math.sin(x));
            k++;
        } while (k <= 7);
        return sum;
    }

    public static int factorial(int num) { //Метод для отримання факторіалу
        int fact = 1;
        for (int i = 1; i <= num; i++) {
            fact *= i;
        }
        return fact;
    }
}
