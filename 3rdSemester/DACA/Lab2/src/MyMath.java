public class MyMath {
    // recursive
    public static int fact_r(int num) {
        if (num <= 1){
            return 1;
        }
        return num * fact_r(num - 1);
    }

    // iterative
    public static int fact_i(int num) {
        int res = 1;
        for (int i = 1; i <= num; i++) {
            res = res * i;
        }
        return res;
    }

    // convert angle in degrees to radians
    public static double toRadians(double degrees) {
        return degrees * Math.PI / 180;
    }

    // my sinus, made by Taylor series
    public static double sin(double radians, double eps) {
        radians = Math.abs(radians % (2 * Math.PI));

        double term = 1.0;      // ith term = x^i / i!
        double sum  = 0.0;      // sum of first i terms in taylor series

        for (int i = 1; Math.abs(Math.sin(radians) - sum) > eps || term != 0.0; i++) {
            term *= (radians / i);
            if (i % 4 == 1) sum += term;
            if (i % 4 == 3) sum -= term;
        }
        return sum;
    }

    // Converting cords from polar system to cartesian (for draw func)
    public static double[] polarToCartesian(double r, double theta) {
        return new double[]{Math.cos(theta) * r, Math.sin(theta) * r};
    }
}