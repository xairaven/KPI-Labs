namespace Lab1;

public class BackSubstitution
{
    public static decimal[] Start(decimal[][] A, decimal[] B, int[] xpos)
    {
        Console.WriteLine("!!! Зворотний хід матриці Гауса !!!\n");
        var result = new decimal[A[0].Length];
        
        Console.WriteLine("Починаємо з останнього рядка");
        for (int i = A.Length - 1, counter = 0; i >= 0; i--, counter++)
        {
            decimal tempRes = B[i];
            Console.WriteLine($"Беремо число B[{i+1}] = {Printer.SignificantFigures(tempRes)}");
            for (int j = 0; j < A[i].Length; j++)
            {
                tempRes -= A[i][j] * result[xpos[j] - 1];
                Console.WriteLine($"Тимчасовий результат: tempRes -= A[{i+1}][{j+1}] * result[{xpos[j]} - 1] = " +
                                  $"{Printer.SignificantFigures(tempRes)}");
            }

            result[xpos[counter] - 1] = tempRes;
            Console.WriteLine($"x{xpos[counter]} = {Printer.SignificantFigures(tempRes)}\n");
        }
        
        Printer.PrintResultArray(result, "!!! МАСИВ РЕЗУЛЬТАТІВ !!!");

        return result;
    }
}