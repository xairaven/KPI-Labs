namespace Lab2;

public class Program
{
    public static void Main(string[] args)
    {
        Console.WriteLine("--- Kovalov Alex, TP-12 ---");
        Console.WriteLine("        -- Lab 2 --      \n");
        
        decimal[][] A = {
            new[] {8.00m, 0.14m, 2.62m, 1.54m},
            new[] {0.15m, 1.24m, 0.74m, 0.18m},
            new[] {2.72m, -1.47m, 2.86m, 1.02m},
            new[] {1.91m, 1.17m, 0.25m, 6.63m}
        };

        decimal[] B = { -4.17m, -3.24m, 38.13m, -1.87m };
        
        Console.WriteLine("A:");
        Printer.Matrix(A);

        Console.WriteLine("B:");
        Printer.Vector(B);
        
        Console.WriteLine();

        Calculations.Iterations(A, B);
    }
}