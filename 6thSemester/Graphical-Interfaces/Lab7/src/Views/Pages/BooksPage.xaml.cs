using System.Windows.Controls;
using Lab7.Context;

namespace Lab7.Views.Pages;

public partial class BooksPage : Page
{
    public BooksPage()
    {
        InitializeComponent();
        
        ReloadGrid();
    }

    public void ReloadGrid()
    {
        using var dbContext = new LibraryDbContext();
        BooksGrid.DataContext = dbContext.Books.ToList();
    }
}