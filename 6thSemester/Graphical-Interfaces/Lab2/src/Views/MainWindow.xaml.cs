using System.Windows;
using Lab2.Views.Task1;

namespace Lab2.Views;

/// <summary>
/// Interaction logic for MainWindow.xaml
/// </summary>
public partial class MainWindow : Window
{
    public MainWindow()
    {
        InitializeComponent();
    }

    private void Task1Grid(object sender, RoutedEventArgs e)
    {
        new GridTask().Show();
    }
    
    private void Task1StackPanel(object sender, RoutedEventArgs e)
    {
        new StackPanelTask().Show();
    }
    
    private void Task1WrapPanel(object sender, RoutedEventArgs e)
    {
        new WrapPanelTask().Show();
    }
    
    private void Task1DockPanel(object sender, RoutedEventArgs e)
    {
        new DockPanelTask().Show();
    }

    private void Task1Canvas(object sender, RoutedEventArgs e)
    {
        new CanvasTask().Show();
    }

    private void Task2TextEditor(object sender, RoutedEventArgs e)
    {
        new Task2.TextEditor().Show();
    }

    private void Task3TextEditor(object sender, RoutedEventArgs e)
    {
        new Task3.TextEditor().Show();
    }

    private void Task4TextEditor(object sender, RoutedEventArgs e)
    {
        new Task4.TextEditor().Show();
    }
}