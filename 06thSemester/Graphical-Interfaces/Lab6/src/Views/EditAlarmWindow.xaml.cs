using System.Windows;
using System.Windows.Controls;
using Lab6.Entities;
using Lab6.Repositories;
using Lab6.Utils;

namespace Lab6.Views;

public partial class EditAlarmWindow : Window
{
    private Alarm _record;
    private Panel _stackPanel;

    public EditAlarmWindow(Panel stackPanel, Alarm record)
    {
        InitializeComponent();

        _record = record;
        _stackPanel = stackPanel;

        TitlePicker.Text = _record.Title;
        DatePicker.MinDate = DateTime.Now.Date;
        TimePicker.Value = DateTime.Now;
    }

    private void CancelEditingAlarm(object sender, RoutedEventArgs e)
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
        
        AlarmRepository.EditRecord(new Guid(_record.Id), title, resultDatetime, _record.IsAlarmEnabled);
        
        Panels.UpdateList(_stackPanel);

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