using System.Text;
namespace Lab1;

public class Utils
{
    public static decimal Max(decimal[] arr, int limiter)
    {
        var max = arr[0];
        for (int i = 0; i < arr.Length - limiter; i++)
        {
            if (arr[i] > max)
            {
                max = arr[i];
            }
        }

        return max;
    }

    public static int IndexOfElement(decimal elem, decimal[] arr, int limiter)
    {
        for (int i = 0; i < arr.Length - limiter; i++)
        {
            if (arr[i].Equals(elem)) return i;
        }

        return -1;
    }
    public static int RowWMax(decimal[][] arr, int limiter)
    {
        var max = arr[limiter][0];
        var row = limiter;
        for (int i = limiter; i < arr.Length; i++)
        {
            var rowMax = Max(arr[i], limiter);
            if (rowMax > max)
            {
                max = rowMax;
                row = i;
            }
        }
        
        return row;
    }

    public static void Swap<T>(int i, int j, T[] arr)
    {
        (arr[i], arr[j]) = (arr[j], arr[i]);
    }

    public static T[][] CopyMatrix<T>(T[][] matrix)
    {
        var copy = new T[matrix.Length][];
        for (int i = 0; i < matrix.Length; i++)
        {
            copy[i] = new T[matrix[0].Length];
        }

        for (int i = 0; i < copy.Length; i++)
        {
            for (int j = 0; j < copy.Length; j++)
            {
                copy[i][j] = matrix[i][j];
            }
        }

        return copy;
    }
    
    public static T[] CopyArray<T>(T[] matrix)
    {
        var copy = new T[matrix.Length];

        for (int i = 0; i < copy.Length; i++)
        {
            copy[i] = matrix[i];
        }

        return copy;
    }
}