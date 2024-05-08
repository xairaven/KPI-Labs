using System.Runtime.InteropServices.JavaScript;
using System.Text;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Documents;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Navigation;
using System.Windows.Shapes;
using System.Windows.Threading;
using Lab6.Controllers;
using Lab6.Services;
using Lab6.Views;
using Lab6;
using Lab6.Context;

namespace Lab3;

/// <summary>
/// Interaction logic for MainWindow.xaml
/// </summary>
public partial class MainWindow : Window
{
    private readonly SettingsContext _context;
    
    public MainWindow()
    {
        _context = StartupService.ConfigureSettings(this);
        
        InitializeComponent();
    }

    private void MainWindow_OnLoaded(object sender, RoutedEventArgs e)
    {
        UpdateClockData(null, null);

        var timer = new TimerService(UpdateClockData, TimeSpan.FromSeconds(1));
        timer.Start(timeSynchronization: true);

        var controller = new AlarmController(_context);
        controller.Start();
    }

    private void UpdateClockData(object? sender, EventArgs? e)
    {
        var dateTime = DateTime.Now;
        
        HourLabel.Content = $"{dateTime.Hour:00}";
        MinuteLabel.Content = $"{dateTime.Minute:00}";
        SecondLabel.Content = $"{dateTime.Second:00}";
        DayLabel.Content = $"{dateTime.DayOfWeek.ToString()[..3]}";
        DateLabel.Content = $"{dateTime.Day:00}/{dateTime.Month:00}/{dateTime.Year:0000}";
    }

    private void App_Exit(object sender, RoutedEventArgs e)
    {
        Application.Current.Shutdown();
    }

    private void Open_Settings(object sender, RoutedEventArgs e)
    {
        new Settings(_context).Show();
    }

    private void Open_AlarmsList(object sender, RoutedEventArgs e)
    {
        new AlarmsList().Show();
    }
}