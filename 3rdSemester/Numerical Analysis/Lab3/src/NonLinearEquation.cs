namespace Lab3;

public class NonLinearEquation
{
    public double Coef { get; }
    public int Degree { get; }
    public NonLinearEquation? Next { get; }

    public NonLinearEquation(double coef, int degree, NonLinearEquation? equation) {
        Coef = coef;
        Degree = degree;
        Next = equation;
    }

    public static void Create(ref NonLinearEquation? equation, int degree)
    {
        for (int i = 0; i < degree + 1; i++) {
            Console.Write($"Input the coef of {i}-th member of equation: ");
            var k = double.Parse(Console.ReadLine()!);

            if (k == 0) continue;
            
            equation = new NonLinearEquation(k, i, equation);
        }
    }
}