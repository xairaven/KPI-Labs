package Lab4.Task1;

import Lab4.Utils.Calculator;
import Lab4.Utils.EquationFunction;

public class Main {
    public static void main(String[] args) {
        double x, y, z;

        EquationFunction functionC = (a, b, c) -> {
            var firstFactor = Math.pow(a, 3);
            var underTgEquation = Math.pow(Math.pow(a, 2) + b, 3);
            var tgEquation = Math.pow(Math.tan(underTgEquation), 3);
            var firstMember = firstFactor * tgEquation;

            var secondMember = c / (a + b);

            return firstMember + secondMember;
        };

        // First test
        System.out.println("First Test:");
        x = 0.2;
        y = 2.5;
        z = 11;
        var calc1 = new Calculator(x, y, z, functionC);
        System.out.println(calc1.formattedResult());

        System.out.println();

        // Second Test
        System.out.println("Second Test:");
        x = 0.6;
        y = 4.1;
        z = 9;
        var calc2 = new Calculator(x, y, z, functionC);
        System.out.println(calc2.formattedResult());
    }
}
