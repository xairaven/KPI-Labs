namespace Lab1;

class Program
{
    public static void Main(string[] args)
    {
        const int var = 22;

        var a = 0.25m * Math.Abs(var - 25);
        var b = 0.35m * (var - 21);

        decimal[][] A =
        {
            new decimal[] { 5.18m + a, 1.12m,     0.95m,     1.32m,     0.83m },
            new decimal[] { 1.12m,     4.28m - a, 2.12m,     0.57m,     0.91m },
            new decimal[] { 0.95m,     2.12m,     6.13m + a, 1.29m,     1.57m },
            new decimal[] { 1.32m,     0.57m,     1.29m,     4.57m - a, 1.25m },
            new decimal[] { 0.83m,     0.91m,     1.57m,     1.25m,     5.21m + a }
        };
        
        decimal[] B = { 6.19m + b, 3.21m, 4.28m - b, 6.25m, 4.95m + b };

        decimal[][] ACopy = Utils.CopyMatrix(A);
        decimal[] BCopy = Utils.CopyArray(B);

        int[] xpos = new int[B.Length];

        ForwardElimination.Start(A, B, xpos);
        decimal[] result = BackSubstitution.Start(A, B, xpos);

        ResidualVector.Start(ACopy, BCopy, result);
    }
}

/*
 Matrix from the video:
 decimal[][] A =
        {
            new decimal[] { 1, 1, -1 },
            new decimal[] { 2, 1, 2 },
            new decimal[] { 1, -3, 1 }
        };
        
 decimal[] B = { 0, 10, -2 };
*/