package Lab4.Utils;

import java.util.ArrayList;
import java.util.List;

public class Calculator {
    double x;
    double y;
    double z;

    EquationFunction function;

    public Calculator(double x, double y, double z, EquationFunction function) {
        this.x = x;
        this.y = y;
        this.z = z;

        this.function = function;
    }

    public double solve() {
        return function.apply(x, y, z);
    }

    public String formattedResult() {
        var result = solve();

        var sb = new StringBuilder();
        sb.append(String.format("x = %.4f %n", x));
        sb.append(String.format("y = %.4f %n", y));
        sb.append(String.format("z = %.4f %n", z));
        sb.append(String.format("Result = %.4f", result));

        return sb.toString();
    }

    public double getX() {
        return x;
    }

    public void setX(double x) {
        this.x = x;
    }

    public double getY() {
        return y;
    }

    public void setY(double y) {
        this.y = y;
    }

    public double getZ() {
        return z;
    }

    public void setZ(double z) {
        this.z = z;
    }

    public EquationFunction getFunction() {
        return function;
    }

    public void setFunction(EquationFunction function) {
        this.function = function;
    }
}
