using System.Windows;
using Lab7.Context;
using Lab7.Repositories;
using Lab7.Utils;

namespace Lab7.Views.Forms;

public partial class CreateForm : Window
{
    private readonly LibraryDbContext _dbContext;

    public CreateForm(LibraryDbContext dbContext)
    {
        InitializeComponent();

        _dbContext = dbContext;
    }

    private void OkButton_OnClick(object sender, RoutedEventArgs e)
    {
        if (!IsValidationPassed()) return;

        try
        {
            new BooksRepository(_dbContext).Create(
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
        
        MessageBox.Show(messageBoxText: "Success! Entry added.",
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