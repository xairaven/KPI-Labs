namespace Lab4;

public static class Calculations
{
    public static double DividedDifference(double[] x, double[] y, int i, int k)
    {
        if (i == k) return y[i];
        
        return (DividedDifference(x, y, i, k - 1) - DividedDifference(x, y, i + 1, k)) / (x[i] - x[k]);
    }
    
    public static double CalcNewtonsPolynom(double x, double[] dd, Polynom[] pn, int n) {
        double rez = dd[0];
        
        for (int i = 1; i < n - 1; i++) {
            if (dd[i] != 0) rez += dd[i] * pn[i - 1].Calculate(x);
        }
        
        return rez;
    }
    
    public static void CalcNewtonsPolynomError(double[] x, double[] dd, Polynom[] pn, int n) {
        double step = (x[1] - x[0]) / 5;
        
        for (int i = 0; i < n * 4.2; i++) {
            double xi = x[0] + step * i;
            Console.WriteLine($" x = {xi:F2}");

            double y = Math.Sin(xi) + Math.Cbrt(2 * xi);
            Console.WriteLine($" y(x) = {y:F10}");

            double p = CalcNewtonsPolynom(xi, dd, pn, n);
            Console.WriteLine($" P(x) = {p}");

            Console.WriteLine($" Error = {Math.Abs(p - y)}");

            Console.WriteLine();
        }
    }
    
    public static void NewtonsPolynom(double[] x, double[] y, int n) {
        var dd = new double[n];
        
        for (int i = 0; i < n; i++) {
            dd[i] = DividedDifference(x, y, 0, i);
        }

        Polynom[] pn = new Polynom[n - 1];
        
        for (int i = 0; i < n - 1; i++) {
            pn[i] = new Polynom(1, 1, null);
            pn[i].Next = new Polynom((-1) * x[i], 0, null);

            if (i <= 0) continue;
            
            pn[i] = pn[i].Product(pn[i - 1]);
            pn[i].Simplify();
        }

        Console.WriteLine(" Newton's polynom:");
        for (int i = 0; i < n; i++)
        {
            if (i == 0)
            {
                Console.Write($" {dd[i]}");
            }
            else if (dd[i] != 0) {
                if (dd[i] > 0) Console.Write(" + ");
                else Console.Write(" - ");
                
                Console.Write("(");
                pn[i - 1].Print();
                Console.Write($") * {Math.Abs(dd[i])}");
            }
        }

        Console.WriteLine("\n");

        CalcNewtonsPolynomError(x, dd, pn, n);
    }
    
    public static void ACoefs(double[] a, double[][] system, double[] y, int n) {
        a[0] = 0.0;
        for (int i = 1; i < n - 2; i++) {
            a[i] = ((-1.0) * system[i - 1][2]) / (system[i - 1][0] * a[i - 1] + system[i - 1][1]);
        }
    }
    
    public static void BCoefs(double[] b, double[] a, double[][] system, double[] y, int n) {
        b[0] = 0.0;
        for (int i = 1; i < n - 2; i++) {
            b[i] = (system[i - 1][3] - system[i - 1][0] * b[i - 1]) / (system[i - 1][0] * a[i - 1] + system[i - 1][1]);
        }
    }
    
    public static double CalcCSpline(double xi, double[] a, double[] b, double[] c, double[] d, double[] x0, int n) {
        double x = 0, ai = 0, bi = 0, ci = 0, di = 0;
	
        for (int i = 1; i < n; i++) {
            x = xi - x0[i - 1];
            ai = a[i - 1];
            bi = b[i - 1];
            ci = c[i - 1];
            di = d[i - 1];
            if (xi < x0[i]) break;
        }
        double rez = ai + bi * x + ci * Math.Pow(x, 2) + di * Math.Pow(x, 3);
        return rez;
    }
    
    public static void CSplinePolynomError(double[] x, double[] a, double[] b, double[] c, double[] d, int n) {
        double step = (x[1] - x[0]) / 5;
        for (int i = 0; i < n * 4.2; i++) {
            double xi = x[0] + step * i;
            Console.WriteLine($" x = {xi:F2}");

            double y = Math.Sin(xi) + Math.Cbrt(2 * xi);
            Console.WriteLine($" y(x) = {y:F10}");

            double p = CalcCSpline(xi, a, b, c, d, x, n);
            Console.WriteLine($" S(x) = {p}");
 
            Console.WriteLine($" Error = {Math.Abs(p - y)}");

            Console.WriteLine();
        }
    }
    
    public static void CSpline(double[] x, double[] y, int n) {
        double[] a = new double[n - 1];
        double[] b = new double[n - 1];
        double[] c = new double[n - 1];
        double[] d = new double[n - 1];
        double[] h = new double[n - 1];

        for (int i = 0; i < n - 1; i++) {
            h[i] = x[i + 1] - x[i];
            if (i == 0 || i == n - 2) c[i] = 0.0;
        }
	
        var system = new double[n - 3][];
        for (int i = 0; i < n - 3; i++) {
            system[i] = new double[4];
            for (int j = 0; j < 4; j++) {
                switch (j) {
                    case 0: system[i][j] = h[i + 1]; break;
                    case 1: system[i][j] = 2 * (h[i + 1] + h[i + 2]); break;
                    case 2: system[i][j] = h[i + 2]; break;
                    case 3: system[i][j] = 3 * (((y[i + 2] - y[i + 1]) / h[i + 2]) - ((y[i + 1] - y[i]) / h[i + 1])); break;
                }
            }
        }
        
        ACoefs(a, system, y, n);
        BCoefs(b, a, system, y, n);
        
        for (int i = 0; i < n - 3; i++) system[i] = null;

        for (int i = n - 3; i > 0; i--) c[i] = a[i] * c[i + 1] + b[i];

        for (int i = 0; i < n - 1; i++) {
            a[i] = y[i];
            if (i == n - 2) {
                d[i] = ((-1.0) * c[i]) / (3 * h[i]);
                b[i] = ((y[i + 1] - y[i]) / h[i]) - ((2 * h[i] * c[i]) / 3);
            } else {
                d[i] = (c[i + 1] - c[i]) / (3 * h[i]);
                b[i] = ((y[i + 1] - y[i]) / h[i]) - ((h[i] * (c[i + 1] + 2 * c[i])) / 3);
            }

            Console.WriteLine($" S{i+1}:");
            
            Console.Write($" {$"a = {a[i]}", 18}");
            Console.WriteLine($" {$"b = {b[i]}", 18}");
            
            Console.Write($" {$"c = {c[i]}", 18}");
            Console.WriteLine($" {$"d = {d[i]}", 18}");
        }
        
        CSplinePolynomError(x, a, b, c, d, n);

        Console.WriteLine();
    }
}