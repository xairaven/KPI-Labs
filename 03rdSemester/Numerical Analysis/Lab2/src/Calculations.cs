namespace Lab2;

public class Calculations
{
    public static void Iterations(decimal[][] A, decimal[] B, decimal eps = 0.00001m, int Length = 4)
    {
        Console.WriteLine("Calculations by method of simple iterations started!\n");
        
        Console.WriteLine("Residual vectors:");

        var X0 = new decimal[Length];         // Start values
        var X = new decimal[Length];          // Result
        var C = new decimal[Length];          // Multiplying A*x
        
        int count = 0;                        // Iteration counter
        
        var temp = new decimal[Length];       // For accuracy
        decimal maxTemp;

        var Residual = new decimal[Length];   // Residual vector 
        
        do
        {
            count++;

            // Method
            X[0] = (B[0] - A[0][1] * X0[1] - A[0][2] * X0[2] - A[0][3] * X0[3]) / A[0][0];
            X[1] = (B[1] - A[1][0] * X0[0] - A[1][2] * X0[2] - A[1][3] * X0[3]) / A[1][1];
            X[2] = (B[2] - A[2][0] * X0[0] - A[2][1] * X0[1] - A[2][3] * X0[3]) / A[2][2];
            X[3] = (B[3] - A[3][0] * X0[0] - A[3][1] * X0[1] - A[3][2] * X0[2]) / A[3][3];
            
            // Subtraction of X
            for (int i = 0; i < temp.Length; i++)
            {
                temp[i] = Math.Abs(X[i] - X0[i]);
            }
            
            // Max from subtraction of results
            maxTemp = temp.Max();
            
            // Init new values
            for (int i = 0; i < Length; i++)
            {
                X0[i] = X[i];
            }
            
            // Residual vector solving
            // C = A * x
            decimal tempValue = 0;
            for (int i = 0; i < Length; i++)
            {
                for (int j = 0; j < Length; j++)
                {
                    tempValue += A[i][j] * X[j];
                }

                C[i] = tempValue;
                tempValue = 0;
            }

            // R = B - C
            for (int i = 0; i < Length; i++)
            {
                Residual[i] = B[i] - C[i];
            }

            Console.Write($"Iteration {count}: \t");
            Printer.Vector(Residual);
        } while (maxTemp > eps);

        Console.WriteLine();
        for (int i = 0; i < X.Length; i++)
        {
            Console.WriteLine($"X[{i + 1}] = {Printer.SignificantFigures(X[i])}");
        }
        Console.WriteLine();


        Console.WriteLine($"Iterations done: {count}");
        Console.WriteLine($"Accuracy: {eps}");
    }
}