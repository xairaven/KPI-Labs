namespace Lab1;

public class ForwardElimination
{
    public static void Start(decimal[][] A, decimal[] B, int[] xpos)
    {
        // filling x array (1 2 3 4 5 ....)
        for (int i = 0; i < xpos.Length; i++)
        {
            xpos[i] = i + 1;
        }
        
        Printer.PrintResultMatrix(A, B, "Початкова матриця, початок прямого ходу методу Гауса");

        // i == limiter
        for (int i = 0; i < A.Length; i++)
        {
            int rowWMax = Utils.RowWMax(A, i);
            Utils.Swap(i, rowWMax, A);                                                     // swap row in A
            Utils.Swap(i, rowWMax, B);                                                     // swap row in B
            
            Printer.PrintResultMatrix(A, B, $"Міняємо місцями {i + 1} і {rowWMax + 1} рядки");

            int initialMaxId = Utils.IndexOfElement(Utils.Max(A[i], i), A[i], i);
            Utils.Swap(A[i].Length - 1 - i, initialMaxId, xpos);                                 // swap x elems
            for (int j = 0; j < A.Length; j++)
            {
                Utils.Swap(A[j].Length - 1 - i, initialMaxId, A[j]);                             // swap elems of arr
            }
            
            Printer.PrintResultMatrix(A, B, $"Змінюємо місцями {initialMaxId + 1} та {A[i].Length - i} стовбці");

            decimal rowMax = A[i][A[i].Length - 1 - i]; 
            for (int j = 0; j < A[i].Length - i; j++)                                        // dividing all elements on max
            {
                A[i][j] /= rowMax;
            }
            B[i] /= rowMax;
            
            Printer.PrintResultMatrix(A, B, 
                $"Ділимо рядок {i + 1} на локальний максимум ({Printer.SignificantFigures(rowMax)})");

            for (int row = i + 1; row < A.Length; row++)                                          // other rows
            {
                var localMax = A[row][A[i].Length - 1 - i];
                
                for (int elem = 0; elem < A[row].Length - i; elem++)                                   // elems of row
                {
                    A[row][elem] -= A[i][elem] * localMax;
                }
                
                B[row] -= B[i] * localMax;
                
                Printer.PrintResultMatrix(A, B, $"Робимо перетворення з рядком {row + 1}");
            }
        }
    }
}