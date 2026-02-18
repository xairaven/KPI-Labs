using System.Windows;
using Lab4.Database;
using Lab4.Validation;

namespace Lab4.Views;

public partial class CreateForm : Window
{
    private readonly MainWindow _window;
    private readonly AdoWrapper _wrapper;

    public CreateForm(MainWindow window)
    {
        _window = window;
        _wrapper = new AdoWrapper();

        InitializeComponent();
    }

    private async void OkButton_OnClick(object sender, RoutedEventArgs e)
    {
        if (!await IsValidationPassed()) return;

        var query = $@"INSERT INTO Books
                        VALUES ('{ISBNBox.Text.Trim()}',
                                '{TitleBox.Text.Trim()}',
                                '{AuthorsBox.Text.Trim()}',
                                '{PubBox.Text.Trim()}',
                                '{PubYearBox.Text.Trim()}')";

        var affectedRows = await _wrapper.ExecuteNonQuery(query);

        MessageBox.Show(messageBoxText: $"Affected rows: {affectedRows}",
            caption: "Create operation results",
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
        var isIsbnExist = (await ValidateFields.IsbnExists(_wrapper, ISBNBox.Text.Trim()));

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