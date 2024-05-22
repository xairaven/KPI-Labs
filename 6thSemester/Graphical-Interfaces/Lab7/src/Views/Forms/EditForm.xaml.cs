using System.Windows;
using Lab7.Context;
using Lab7.Entities;
using Lab7.Repositories;
using Lab7.Utils;

namespace Lab7.Views.Forms;

public partial class EditForm : Window
{
    private readonly LibraryDbContext _dbContext;
    private readonly string _initIsbn;

    public EditForm(LibraryDbContext dbContext, string initIsbn)
    {
        InitializeComponent();

        _dbContext = dbContext;
        _initIsbn = initIsbn;

        InitializeFields();
    }

    private void InitializeFields()
    {
        var book = _dbContext.Books.Find(_initIsbn);

        if (book is null)
        {
            MessageBox.Show(messageBoxText: "Please, reload page and then try to edit row.",
                caption: "Error!",
                button: MessageBoxButton.OK,
                icon: MessageBoxImage.Error,
                defaultResult: MessageBoxResult.OK);

            return;
        }

        var publisher = _dbContext.Publishers.Find(book.PublisherCode);
        if (publisher is null)
        {
            MessageBox.Show(messageBoxText: "Please, reload page and then try to edit row.",
                caption: "Error!",
                button: MessageBoxButton.OK,
                icon: MessageBoxImage.Error,
                defaultResult: MessageBoxResult.OK);

            return;
        }

        ISBNBox.Text = book.Isbn;
        TitleBox.Text = book.Title;
        AuthorsBox.Text = book.Authors;
        PubBox.Text = publisher.Name;
        PubYearBox.Text = book.PublicationYear.ToString() ?? string.Empty;
    }

    private void OkButton_OnClick(object sender, RoutedEventArgs e)
    {
        if (!IsValidationPassed()) return;

        try
        {
            new BooksRepository(_dbContext).Update(
                _initIsbn,
                ISBNBox.Text.Trim(),
                TitleBox.Text.Trim(),
                AuthorsBox.Text.Trim(),
                PubBox.Text.Trim(),
                PubYearBox.Text.Trim()
            );
        }
        catch (Exception exception)
        {
            MessageBox.Show(messageBoxText: exception.Message,
                caption: "Error!",
                button: MessageBoxButton.OK,
                icon: MessageBoxImage.Error,
                defaultResult: MessageBoxResult.OK);
            return;
        }

        MessageBox.Show(messageBoxText: "Success! Entry edited.",
            caption: "Entry added.",
            button: MessageBoxButton.OK,
            icon: MessageBoxImage.Information,
            defaultResult: MessageBoxResult.OK);

        Close();
    }

    private void CancelButton_OnClick(object sender, RoutedEventArgs e)
    {
        Close();
    }

    private bool IsValidationPassed()
    {
        return !ValidateFields.IsEmpty(ISBNBox.Text) &&
               !ValidateFields.IsEmpty(TitleBox.Text) &&
               !ValidateFields.IsEmpty(AuthorsBox.Text) &&
               !ValidateFields.IsEmpty(PubBox.Text) &&
               !ValidateFields.IsEmpty(PubYearBox.Text) &&
               ValidateFields.CanCastYear(PubYearBox.Text.Trim());
    }
}