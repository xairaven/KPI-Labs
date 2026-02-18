import edu.princeton.cs.algs4.StdDraw;
import java.util.Scanner;

/**
 * Лабораторна робота №2
 * Ознайомитись з рекурсивним викликом функції. Розробити алгоритм
 * розрахунку значення функції за її розкладенням у ряд за умови отримання
 * результату з заданою точністю. Врахувати діапазон дозволених значень для
 * змінної x.
 * У якості індивідуального завдання необхідно написати програмний код, що
 * реалізує алгоритм розрахунку значень функцій за їх розкладенням в ряд із
 * заданою користувачем точністю.
 * Використати математичні моделі геометричних фігур з Додатку В-2.
 * Завдання обрати згідно свого варіанта.
 * Індивідуальне завдання:
 * Трипелюсткова троянда: p = a*sin(3*phi), phi є (-inf; +inf)
 *
 * Виконав студент групи ТР-12
 * Ковальов Олександр
 * Номер варіанту: 22
 */
public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);

        System.out.println("Function: (p = a * sin (3*phi))");

        System.out.println("Set the angle value in degrees (-infinity - +infinity):");
        int degrees = sc.nextInt();

        System.out.println("Set number a:");
        int a = sc.nextInt();

        System.out.println("Set epsilon (accuracy) (use \".\" instead of \",\" while input):");
        double eps = sc.nextDouble();

        double radians = MyMath.toRadians(degrees);

        double myP = a * MyMath.sin(3*radians, eps);
        double javaP = a * Math.sin(3*radians);

        System.out.printf("\nFunction: p = %d*sin(3 * %.3f)\n", a, radians);
        System.out.println("a = " + a);
        System.out.println("phi = " + degrees + " degrees");

        System.out.println("p (my result) = " + myP);
        System.out.println("p (java compiler sinus) = " + javaP);
        System.out.println("Fault: " + Math.abs(myP - javaP));
        draw(a);
    }

    public static void draw(int a) {
        StdDraw.setXscale(-5,5);
        StdDraw.setYscale(-5,5);
        StdDraw.setPenRadius(0.005);
        StdDraw.setPenColor(StdDraw.BOOK_RED);

        for (double i = 0; i < 360; i += 0.1) {
            double[] first = MyMath.polarToCartesian(a*MyMath.sin(Math.toRadians(3*i), 0.01), MyMath.toRadians(i));
            StdDraw.point(first[0], first[1]);
        }
   }
}