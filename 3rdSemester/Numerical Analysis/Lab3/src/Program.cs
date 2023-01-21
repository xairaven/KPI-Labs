namespace Lab3;

public class Program
{
    public static void Main(string[] args)
    {
        Console.WriteLine("--- Kovalov Alex, TP-12 ---");
        Console.WriteLine("        -- Lab 3 --      \n");

        Console.Write("Input the degree of equation: ");
        int n = int.Parse(Console.ReadLine()!);

        Console.WriteLine();
        
        NonLinearEquation? equation = null;
        NonLinearEquation.Create(ref equation, n);
        
        if (equation == null)
        {
            Console.WriteLine("No equation");
            return;
        }

        Console.WriteLine();
        Printer.Equation(equation);
        Console.WriteLine();

        Console.Write("Input the precision: ");
        var precision = double.Parse(Console.ReadLine()!);
        
        Console.Write("Input the left border a: ");
        var a = double.Parse(Console.ReadLine()!);
        
        Console.Write("Input the right border b: ");
        var b = double.Parse(Console.ReadLine()!);

        Console.WriteLine();
        
        Calculations.Bisection(equation, a, b, precision);
        Calculations.Chord(equation, a, b, precision);
        Calculations.Newtons(equation, a, b, precision);
    }
}