using System.Globalization;
using System.Windows;
using Lab6.Views.Controls;
using Lab6.Repositories;

namespace Lab6.Views;

public partial class AddAlarmWindow : Window
{
    private AlarmsList _listWindow;
    
    public AddAlarmWindow(AlarmsList window)
    {
        InitializeComponent();
        DatePicker.MinDate = DateTime.Now.Date;
        TimePicker.Value = DateTime.Now;

        _listWindow = window;
    }

    private void CancelAddAlarm(object sender, RoutedEventArgs e)
    {
        Close();
    }

    private void SaveAlarm(object sender, RoutedEventArgs e)
    {
        var title = TitlePicker.Text.Trim();
        var time = TimePicker.Value!.Value.TimeOfDay;
        var date = DatePicker.Value!.Value.Date;

        if (!CheckIsAlarmReal(time, date))
        {
            MessageBox.Show(messageBoxText:"Error! Setting alarms for the past is strictly prohibited.", 
                caption: "Error!", 
                button: MessageBoxButton.OK, 
                icon: MessageBoxImage.Error, 
                defaultResult: MessageBoxResult.OK);
            return;
        }

        var resultDatetime = date.Date.Add(time);

        AlarmRepository.AddRecord(title, resultDatetime);
        
        _listWindow.UpdateList();

        Close();
    }

    private bool CheckIsAlarmReal(TimeSpan time, DateTime date)
    {
        // If date on DatePicker is later than datetime Now, than any time is good
        if (date.CompareTo(DateTime.Now.Date) == 1)
        {
            return true;
        }

        // But if date on DatePicker is equal (real) or earlier (not real) than datetime.now: 
        // Compare TimePicker and Datetime.now.
        // If TimePicker is later (> 0) than DateTime.now, its real alarm
        return time.CompareTo(DateTime.Now.TimeOfDay) > 0;
    }
}