namespace Lab4;

public class Program
{
    public static void Main(string[] args)
    {
        Console.WriteLine("--- Kovalov Alex, TP-12 ---");
        Console.WriteLine("        -- Lab 4 --      \n");

        Console.WriteLine(" Function: y(x) = sin(x) + (2*x)^(1/3)");
        Console.WriteLine(" Interpolation nodes: -4, -2, 0, 2, 4");

        Console.WriteLine();

        int n = 5;
        var x = new double[n];
        var y = new double[n];
        var step = -4.0;
        for (int i = 0; i < n; i++) {
            x[i] = step;
            step += 2;
            y[i] = Math.Sin(x[i]) + Math.Cbrt(2 * x[i]);
        }
        
        Calculations.NewtonsPolynom(x, y, n);
        Calculations.CSpline(x, y, n);
    }
}