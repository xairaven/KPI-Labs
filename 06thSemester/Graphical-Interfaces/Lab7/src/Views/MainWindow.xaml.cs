using System.Windows;
using System.Windows.Controls;
using Lab7.Context;
using Lab7.Views.Pages;

namespace Lab7.Views;

/// <summary>
/// Interaction logic for MainWindow.xaml
/// </summary>
public partial class MainWindow : Window
{
    private readonly Dictionary<string, Page> _pages;
    
    public MainWindow()
    {
        InitializeComponent();

        _pages = new Dictionary<string, Page>();
        CreatePages();
        
        MainFrame.Navigate(_pages["MainPage"]);
    }

    private void CreatePages()
    {
        _pages["MainPage"] = new MainPage(MainFrame, _pages);
        _pages["BooksPage"] = new BooksPage();
        _pages["PublishersPage"] = new PublishersPage();
    }
}