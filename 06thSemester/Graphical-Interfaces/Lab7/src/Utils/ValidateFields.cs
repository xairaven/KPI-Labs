using System.Windows;
using Lab7.Context;

namespace Lab7.Utils;

public static class ValidateFields
{
    public static bool IsbnExists(LibraryDbContext dbContext, string isbn, string initIsbn = "")
    {
        var books = dbContext.Books.ToList();
        foreach (var book in books)
        {
            if (book.Isbn.Equals(initIsbn.Trim())) continue;

            if (!book.Isbn.Equals(isbn)) continue;
            
            return true;
        }

        return false;
    }

    public static bool CanCastYear(string year)
    {
        var canBeCasted = short.TryParse(year, out _);

        if (!canBeCasted)
        {
            MessageBox.Show(messageBoxText: "Something wrong with year",
                caption: "Error!",
                button: MessageBoxButton.OK,
                icon: MessageBoxImage.Error,
                defaultResult: MessageBoxResult.OK);
        }

        return canBeCasted;
    }
    
    public static bool IsEmpty(string field)
    {
        var isEmpty = field.Trim().Equals("");

        if (isEmpty)
        {
            MessageBox.Show(messageBoxText: "Some field is empty.",
                caption: "Error!",
                button: MessageBoxButton.OK,
                icon: MessageBoxImage.Error,
                defaultResult: MessageBoxResult.OK);
        }

        return isEmpty;
    }
}