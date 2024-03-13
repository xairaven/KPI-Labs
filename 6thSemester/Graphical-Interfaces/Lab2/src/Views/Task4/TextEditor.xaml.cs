using System.IO;
using System.Windows;
using System.Windows.Input;
using Microsoft.Win32;

namespace Lab2.Views.Task4;

public partial class TextEditor : Window
{
    public TextEditor()
    {
        InitializeComponent();
        
        var openBinding = new CommandBinding(ApplicationCommands.Open,
            Execute_Open, CanExecute_Open);
        var closeBinding = new CommandBinding(ApplicationCommands.Close,
            Execute_Close, CanExecute_Close);
        var saveBinding = new CommandBinding(ApplicationCommands.Save, 
            Execute_Save, CanExecute_Save);

        CommandBindings.Add(openBinding);
        CommandBindings.Add(closeBinding);
        CommandBindings.Add(saveBinding);
    }

    private void CanExecute_Save(object sender, CanExecuteRoutedEventArgs e)
    {
        e.CanExecute = MainTextBox.Text.Trim().Length > 0;
    }

    private void Execute_Save(object sender, ExecutedRoutedEventArgs e)
    {
        File.WriteAllText("Lab2File.txt", MainTextBox.Text);
        MessageBox.Show("The file was saved!");
    }
    
    private void CanExecute_Open(object sender, CanExecuteRoutedEventArgs e)
    {
        e.CanExecute = MainTextBox.Text.Trim().Length == 0;
    }

    private void Execute_Open(object sender, ExecutedRoutedEventArgs e)
    {
        const string filter = "Text files (*.txt)|*.txt;|All files (*.*)|*.*";
        
        var openDialog = new OpenFileDialog()
        {
            InitialDirectory = AppDomain.CurrentDomain.BaseDirectory,
            Filter = filter
        };

        if (openDialog.ShowDialog() == true)
        {
            MainTextBox.Text = File.ReadAllText(openDialog.FileName);
        }
    }
    
    private void CanExecute_Close(object sender, CanExecuteRoutedEventArgs e)
    {
        e.CanExecute = MainTextBox.Text.Length > 0;
    }

    private void Execute_Close(object sender, ExecutedRoutedEventArgs e)
    {
        MainTextBox.Text = "";
    }
}