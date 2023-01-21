using System.Text;

namespace Lab2;

public class Printer
{
    static Printer()
    {
        Console.OutputEncoding = Encoding.UTF8;
    }
    
    public static void Matrix(decimal[][] arr, params string[] messages)
    {
        if (messages.Length == 1) Console.WriteLine($"/// {messages[0]} ///");
        foreach (var row in arr)
        {
            for (int i = 0; i < arr.Length; i++)
            {
                Console.Write($"x{i + 1} = {SignificantFigures(row[i])}\t");
            } 
            Console.WriteLine();
        }
        Console.WriteLine();
    }
    
    public static void Vector(decimal[] arr, params string[] messages)
    {
        if (messages.Length == 1) Console.WriteLine($"/// {messages[0]} ///");
        
        foreach (var t in arr)
        {
            Console.Write($"{SignificantFigures(t)}  ");
        } 
        Console.WriteLine();
    }

    public static string SignificantFigures(decimal number)
    {
        if (number == 0) return "0";
        
        const int significant = 6;
        
        var numberString = $"{number}";
        var charArray = numberString.ToCharArray();
        var sb = new StringBuilder();

        int integer = 0;
        int fractional = 0;
        int comaPos = 0;

        for (int i = 0; i < charArray.Length; i++)
        {
            if (charArray[i] == ',') comaPos = i;
            
            if (charArray[i] == '-' || comaPos <= 0)
            {
                integer++;
            }
            else
            {
                fractional++;
            }
        }

        int possibleZeros = significant - fractional - integer;
        
        for (int i = 0, current = 0; i < significant && i < charArray.Length; i++)
        {
            if (i == 0 && charArray[i] == '-')
            {
                sb.Append('-');
                current++;
                continue;
            }
            
            for (; possibleZeros >= 1; possibleZeros--)
            {
                sb.Append('0');
                current++;
            }

            if (current != significant - 1)
            {
                sb.Append(charArray[i]);
                current++;
            }
            else if (i + 1 < charArray.Length)
            {
                if (int.Parse(charArray[i + 1].ToString()) > 5)
                {
                    sb.Append((int.Parse(charArray[i].ToString()) + 1) % 10);
                }
                else sb.Append(charArray[i]);
            }
            else
            {
                sb.Append(charArray[i]);
            }
        }

        return sb.ToString();
    }
}