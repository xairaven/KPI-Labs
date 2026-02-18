namespace Lab3;

public class Printer
{
    public static void Member(double coef, int degree) {
        if (coef != 1 && degree != 0)
        {
            Console.Write($"{coef}x^{degree}");
        }
        else if (degree != 0)
        {
            Console.Write($"x^{degree}");
        }
        else
        {
            Console.Write(coef);
        }
    }
    
    public static void Equation(NonLinearEquation? equation) {
        if (equation == null) {
            Console.WriteLine("No equation.");
            return;
        }

        Console.Write("Equation:   ");
        Member(equation.Coef, equation.Degree);
        var item = equation.Next;

        while (item != null)
        {
            Console.Write(item.Coef > 0 ? " + " : " - ");
            Member(Math.Abs(item.Coef), item.Degree);
            item = item.Next;
        }
        Console.WriteLine();
    }
}