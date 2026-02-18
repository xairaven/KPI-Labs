namespace Lab1;

public class ResidualVector
{
    public static void Start(decimal[][] A, decimal[] B, decimal[] x)
    {
        Console.WriteLine("!!! Розв'язування вектора нев'язки !!!");
        Console.WriteLine("Формула: r = B - Ax\n");
        
        Printer.PrintResultMatrix(A, B, "Розширена матриця");
        Printer.PrintResultArray(x, "Вектор результатів");

        decimal[] r = Subtract(B, Mult(A, x));
        Console.WriteLine("!!! Вектор нев'язки !!!");
        Console.Write("[");
        foreach (var elem in r)
        {
            Console.Write(Printer.SignificantFigures(elem) + "\t");
        }
        Console.WriteLine("]");
    }
    
    // multiplication matrix * vector
    public static decimal[] Mult(decimal[][] a, decimal[] b)
    {
        if (a[0].Length != b.Length) throw new ArithmeticException("a[0].Length != b.Length");
        
        
        var result = new decimal[a.Length];

        for (int i = 0; i < a.Length; i++) {
            for (int j = 0; j < b.Length; j++) {
                decimal expr = 0;
                for (int k = 0; k < a[0].Length; k++) {
                    expr += a[i][k] * b[k];
                }
                result[i] = expr;
            }
        }
        return result;
    }
    
    // subtracting vectors
    public static decimal[] Subtract(decimal[] a, decimal[] b)
    {
        if (a.Length != b.Length && a.Length != b.Length) throw new ArithmeticException("a[0].Length != b.Length");
        
        
        var result = new decimal[a.Length];

        for (int i = 0; i < a.Length; i++) {
            result[i] = a[i] - b[i];
        }
        return result;
    }
}