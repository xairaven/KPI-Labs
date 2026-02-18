using System.Globalization;
using System.Windows.Controls;
using Lab6.Repositories;
using Lab6.Views.Controls;

namespace Lab6.Utils;

public static class Panels
{
    public static void UpdateList(Panel panel)
    {
        AlarmRepository.CheckAlarmRelevance();
        
        panel.Children.Clear();
        foreach (var record in AlarmRepository.AlarmSet)
        {
            var element = new AlarmElement
            {
                Id = new Guid(record.Id),
                Title = record.Title
            };

            var datetime = record.Datetime;
            element.Time = $"{datetime.Hour:00}:{datetime.Minute:00}";
            
            var monthName = datetime.ToString("MMM", CultureInfo.InvariantCulture);
            element.Date = $"{datetime.DayOfWeek.ToString()[..3]}, {datetime.Day:00} {monthName}";
            
            element.IsAlarmEnabled = record.IsAlarmEnabled;
            
            panel.Children.Add(element);
        }
    }
}