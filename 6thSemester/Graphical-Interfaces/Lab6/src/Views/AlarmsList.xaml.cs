using System.Globalization;
using System.Windows;
using Lab6.Repositories;
using Lab6.Utils;
using Lab6.Views.Controls;

namespace Lab6.Views;

public partial class AlarmsList : Window
{
    public AlarmsList()
    {
        InitializeComponent();
        
        UpdateList();
    }

    private void AddAlarmShow(object sender, RoutedEventArgs e)
    {
        new AddAlarmWindow(this).Show();
    }
    
    public void UpdateList()
    {
        Panels.UpdateList(ListPanel);
    }
}