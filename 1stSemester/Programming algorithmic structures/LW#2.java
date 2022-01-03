public class Lr21 {
    public static void main(String[] args) {
        double x1 = -2.3;
        double x2 = 0.6;
        double x3 = 4.8;

        System.out.println("-- Завдання 1 --:\n");
        double a =  0.2;
        double b = 0.9;
        double z = 1.7;
        System.out.printf("Вхідні дані для всіх трьох x:\n a = %.4f\t b =  %.4f\t z = %.4f\n\nРезультати:\n",a,b,z);
        System.out.printf("y1: %.4f\tx1:%.4f\n", task1(x1,a,b,z), x1);
        System.out.printf("y2: %.4f\tx2:%.4f\n", task1(x2,a,b,z), x2);
        System.out.printf("y3: %.4f\tx3:%.4f\n", task1(x3,a,b,z), x3);

        System.out.println("\n-- Завдання 2 --:");
        double[] x = {x1,x2,x3};
        for (int i = 0;i<3;i++) {
            System.out.printf("\nПри х%d (%.4f):\n", i+1, x[i]);
            for (int j = 0;j<3;j++) {
                switch (j) {
                    case 0:
                        a = 1.5;
                        b = 6.4;
                        z = Math.log(Math.abs(b*Math.pow(x[i],3)+1.5));
                        System.out.printf("\tПри умові a = %.4f\t b =  %.4f\t z = %.4f\n",a,b,z);
                        System.out.printf("\ty:%.4f\n", task2(x[i],a,b,z));
                        break;
                    case 1:
                        a = 1.9;
                        b = 8.6;
                        z = Math.log(Math.abs(b*Math.pow(x[i],3)+3));
                        System.out.printf("\tПри умові a = %.4f\t b =  %.4f\t z = %.4f\n",a,b,z);
                        System.out.printf("\ty:%.4f\n", task2(x[i],a,b,z));
                        break;
                    case 2:
                        a = 0.6;
                        b = 2.4;
                        z = Math.log(Math.abs(b*Math.pow(x[i],3)+1.8));
                        System.out.printf("\tПри умові a = %.4f\t b =  %.4f\t z = %.4f\n",a,b,z);
                        System.out.printf("\ty:%.4f\n", task2(x[i],a,b,z));
                        break;
                }
            }
        }
    }

    public static double task1(double x, double a, double b, double z) {
        //використання конструкції if
        double y = 0;
        if (x<a&&x>=0) {
            y = a + b*x + Math.pow((Math.sin(z*Math.pow(x,3.5))),2);
        } else if (a<=x&&x<=b) {
            y = a + Math.log(Math.abs(a*b-z*x));
        } else if (b<x) {
            y = Math.sqrt(Math.abs(a+1.0/Math.tan(z*x))) + b*x;
        } else if (x<a&&x<0) { //Додаткова умова - при х<a і x = -2.3 умова не відповідає ОДЗ (x^3.5), x<0
            y = a + b*x + Math.pow((Math.sin(z*Math.pow(x,3))),2);
        }
        return y;
    }

    public static double task2(double x, double a, double b, double z) {
        double y = 0;
        if (Math.pow(a,3)<x&&x<=b) {
            y = Math.log(Math.abs(b*z*x))+z*Math.pow(a,2.5);
        } else if (x>b) {
            y = a*x*x+b*Math.pow(z,a)+Math.pow(Math.sin(z*x),2);
        } else if (x<=Math.pow(a,3)) {
            y = Math.cos(a*x+b) + Math.log(Math.abs(z*x));
        }
        return y;
    }
}
