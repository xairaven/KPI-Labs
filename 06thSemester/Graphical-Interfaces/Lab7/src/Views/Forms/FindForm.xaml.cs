using System.Net.Mime;
using System.Windows;
using System.Windows.Controls;
using Lab7.Context;
using Lab7.Entities;
using Lab7.Repositories;
using Lab7.Views.Pages;
using LinqKit;

namespace Lab7.Views.Forms;

public partial class FindForm : Window
{
    private readonly LibraryDbContext _dbContext;
    private DataGrid _grid;

    public FindForm(LibraryDbContext dbContext, DataGrid grid)
    {
        InitializeComponent();

        _dbContext = dbContext;
        _grid = grid;
    }

    private void OkButton_OnClick(object sender, RoutedEventArgs e)
    {
        var isbn = ISBNBox.Text.Trim();
        var title = TitleBox.Text.Trim();
        var authors = AuthorsBox.Text.Trim();
        var publisher = PubBox.Text;
        var year = Convert.ToInt16(ZeroIfEmpty(PubYearBox.Text.Trim()));
        
        var predicate = PredicateBuilder.New<Book>();

        if (!isbn.Equals("")) predicate.And(b => b.Isbn.Contains(isbn));
        if (!title.Equals("")) predicate.And(b => b.Title.Contains(title));
        if (!authors.Equals("")) predicate.And(b => b.Authors.Contains(authors));
        if (year != 0) predicate.And(b => b.PublicationYear == year);
        if (!publisher.Equals(""))
        {
            var publishers = _dbContext.Publishers.Where(p => p.Name.Contains(publisher));

            foreach (var publisherTemp in publishers)
            {
                predicate.Or(b => b.PublisherCode == publisherTemp.Id);   
            }
        }
        
        var result = _dbContext.Books.Where(predicate);
        
        var joined = result.Join(_dbContext.Publishers,
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

        _grid.DataContext = joined.ToList();
    }

    private void CancelButton_OnClick(object sender, RoutedEventArgs e)
    {
        Close();
    }
    
    private static string ZeroIfEmpty(string s)
    {
        return string.IsNullOrEmpty(s) ? "0" : s;
    }
}