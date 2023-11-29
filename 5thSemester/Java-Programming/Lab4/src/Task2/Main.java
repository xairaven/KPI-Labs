package Lab4.Task2;

import Lab4.Utils.Calculator;
import Lab4.Utils.EquationFunction;

public class Main {
    public static void main(String[] args) {
        double x, y, z;

        EquationFunction functionL = (a, b, c) -> {
            var numerator = Math.pow(a, 2) * (a + 2);
            var denominator = c - Math.pow(Math.sin(a + b), 2);

            return numerator / denominator;
        };

        EquationFunction functionN = (a, b, c) -> {
            var firstMember = Math.sqrt(a / b);
            var secondMember = Math.pow(Math.cos(a + c), 2);

            return firstMember + secondMember;
        };

        // First test
        System.out.println("First Test:");
        x = 0.1;
        y = 0.01;
        z = 0.7;
        var calc1 = new Calculator(x, y, z, functionL);
        System.out.println(calc1.formattedResult());
        System.out.println();
        calc1.setFunction(functionN);
        System.out.println(calc1.formattedResult());

        System.out.println("\n\n");

        // Second Test
        System.out.println("Second Test:");
        x = 1.1;
        y = -1.2;
        z = 4.4;
        var calc2 = new Calculator(x, y, z, functionL);
        System.out.println(calc2.formattedResult());
        System.out.println();
        calc2.setFunction(functionN);
        System.out.println(calc2.formattedResult());
    }
}
