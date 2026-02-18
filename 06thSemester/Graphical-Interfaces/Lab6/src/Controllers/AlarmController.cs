using Lab6;
using Lab6.Context;
using Lab6.Repositories;
using Lab6.Services;
using Lab6.Views;

namespace Lab6.Controllers;

public class AlarmController
{
    private SettingsContext _context;

    public AlarmController(SettingsContext context)
    {
        _context = context;
    }
    
    public void Start()
    {
        var timer = new TimerService(CheckAlarms, TimeSpan.FromSeconds(3));

        timer.Start();
    }

    private void CheckAlarms(object? sender, EventArgs e)
    {
        foreach (var record in AlarmRepository.AlarmSet)
        {
            if (!record.IsAlarmEnabled) continue;
            
            var truncatedNow = Truncate(DateTime.Now, TimeSpan.FromMinutes(1));
            var truncatedAlarmTime = Truncate(record.Datetime, TimeSpan.FromMinutes(1));

            if (truncatedNow.CompareTo(truncatedAlarmTime) != 0) continue;
            
            AlarmRepository.EditRecord(new Guid(record.Id), 
                record.Title, record.Datetime, !record.IsAlarmEnabled);
            new AlarmNotification(_context, record).Show();
        }
    }
    
    private static DateTime Truncate(DateTime dateTime, TimeSpan timeSpan)
    {
        if (timeSpan == TimeSpan.Zero) return dateTime; // Or could throw an ArgumentException

        // Some comments suggest removing the following line.  I think the check
        // for MaxValue makes sense - it's often used to represent an indefinite expiry date.
        // (The check for DateTime.MinValue has no effect, because DateTime.MinValue % timeSpan
        // is equal to DateTime.MinValue for any non-zero value of timeSpan.  But I think
        // leaving the check in place makes the intent clearer).
        // YMMV and the fact that different people have different expectations is probably
        // part of the reason such a method doesn't exist in the Framework.
        if (dateTime == DateTime.MinValue || dateTime == DateTime.MaxValue) return dateTime; // do not modify "guard" values

        return dateTime.AddTicks(-(dateTime.Ticks % timeSpan.Ticks));
    }
}