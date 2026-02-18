using System.Data;
using System.Windows;
using Lab4.Database;
using Lab4.Validation;

namespace Lab4.Views;

public partial class UpdateForm : Window
{
    private readonly string _initIsbn;
    private readonly MainWindow _window;
    private readonly AdoWrapper _wrapper;

    public UpdateForm(DataRowView row, MainWindow window)
    {
        _window = window;
        _wrapper = new AdoWrapper();
        
        InitializeComponent();

        _initIsbn = row["isbn"].ToString()!;

        ISBNBox.Text = _initIsbn;
        TitleBox.Text = row["title"].ToString()!;
        AuthorsBox.Text = row["authors"].ToString()!;
        PubBox.Text = row["publisher"].ToString()!;
        PubYearBox.Text = row["publication_year"].ToString()!;
    }

    private async void OkButton_OnClick(object sender, RoutedEventArgs e)
    {
        if (!await IsValidationPassed()) return;
        
        var query = @$"UPDATE Books
                        SET isbn = '{ISBNBox.Text.Trim()}',
                            title = '{TitleBox.Text.Trim()}',
                            authors = '{AuthorsBox.Text.Trim()}',
                            publisher = '{PubBox.Text.Trim()}',
                            publication_year = '{PubYearBox.Text.Trim()}'
                        WHERE isbn = '{_initIsbn}'";
        
        var affectedRows = await _wrapper.ExecuteNonQuery(query);
        
        MessageBox.Show(messageBoxText: $"Affected rows: {affectedRows}",
            caption: "Update operation results",
            button: MessageBoxButton.OK,
            icon: MessageBoxImage.Information,
            defaultResult: MessageBoxResult.OK);
        
        await _window.LoadListBox();
        
        Close();
    }

    private void CancelButton_OnClick(object sender, RoutedEventArgs e)
    {
        Close();
    }

    private async Task<bool> IsValidationPassed()
    {
        var isIsbnExist = (await ValidateFields.IsbnExists(_wrapper, ISBNBox.Text.Trim(), _initIsbn));
        
            return !IsThereEmptyField() &&
                   !isIsbnExist &&
                   ValidateFields.CanCastYear(PubYearBox.Text.Trim());
    }
    
    private bool IsThereEmptyField()
    {
        return ValidateFields.IsEmpty(ISBNBox.Text) ||
        ValidateFields.IsEmpty(TitleBox.Text) ||
        ValidateFields.IsEmpty(AuthorsBox.Text) ||
        ValidateFields.IsEmpty(PubBox.Text) ||
        ValidateFields.IsEmpty(PubYearBox.Text);
    }
}