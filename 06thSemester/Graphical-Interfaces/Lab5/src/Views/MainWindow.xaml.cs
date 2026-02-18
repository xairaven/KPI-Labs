using System.Text.RegularExpressions;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Input;
using Lab5.Context;
using Lab5.Entities;
using LinqKit;
using Microsoft.IdentityModel.Tokens;

namespace Lab5.Views;

/// <summary>
/// Interaction logic for MainWindow.xaml
/// </summary>
public partial class MainWindow : Window
{
    public MainWindow()
    {
        InitializeComponent();

        using var dbContext = new LibraryDbContext();
        BooksGrid.DataContext = dbContext.Books.ToList();
        PublishersGrid.DataContext = dbContext.Publishers.ToList();

        InitializeJoinedTable(dbContext);
    }

    private void InitializeJoinedTable(LibraryDbContext dbContext)
    {
        var joined = dbContext.Books.Join(dbContext.Publishers,
            x => x.PublisherCode,
            y => y.Id,
            (x, y) => new
            {
                x.Isbn,
                x.Title,
                x.Authors,
                Publisher = y.Name,
                x.PublicationYear
            });

        JoinedGrid.DataContext = joined.ToList();
    }

    private void SearchBooksTextChanged(object sender, TextChangedEventArgs e)
    {
        using var dbContext = new LibraryDbContext();

        var isbn = IsbnBox.Text.Trim();
        var title = TitleBox.Text.Trim();
        var authors = AuthorsBox.Text.Trim();
        var code = Convert.ToDecimal(ZeroIfEmpty(PublisherCodeBox.Text.Trim()));
        var year = Convert.ToDecimal(ZeroIfEmpty(PublicationYearBox.Text.Trim()));
        
        var predicate = PredicateBuilder.New<Book>();

        if (!isbn.IsNullOrEmpty()) predicate.And(b => b.Isbn.Contains(isbn));
        if (!title.IsNullOrEmpty()) predicate.And(b => b.Title.Contains(title));
        if (!authors.IsNullOrEmpty()) predicate.And(b => b.Authors.Contains(authors));
        if (code != 0) predicate.And(b => b.PublisherCode == code);
        if (year != 0) predicate.And(b => b.PublicationYear == year);
        
        SearchBooksGrid.DataContext = dbContext.Books.Where(predicate).ToList();
    }
    
    private void SearchByPublisher(object sender, TextChangedEventArgs e)
    {
        using var dbContext = new LibraryDbContext();
        
        var publisher = PublisherSearchBox.Text.Trim();
        
        var joined = dbContext.Books.Join(dbContext.Publishers,
            x => x.PublisherCode,
            y => y.Id,
            (x, y) => new
            {
                x.Isbn,
                x.Title,
                x.Authors,
                Publisher = y.Name,
                x.PublicationYear
            });

        SearchByPublisherGrid.DataContext = joined
            .Where(b => b.Publisher.Contains(publisher))
            .ToList();
    }
    
    private static string ZeroIfEmpty(string s)
    {
        return string.IsNullOrEmpty(s) ? "0" : s;
    }
    
    private void NumberValidationTextBox(object sender, TextCompositionEventArgs e)
    {
        var regex = new Regex("[^0-9]+");
        e.Handled = regex.IsMatch(e.Text);
    }
}