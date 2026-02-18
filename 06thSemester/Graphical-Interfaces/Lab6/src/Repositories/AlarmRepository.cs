using Lab6.Context;
using Lab6.Entities;
using Microsoft.EntityFrameworkCore;

namespace Lab6.Repositories;

public static class AlarmRepository
{
    private static ApplicationDbContext _dbContext;
    
    public static DbSet<Alarm> AlarmSet { get; }

    static AlarmRepository()
    {
        _dbContext = new ApplicationDbContext();
        AlarmSet = _dbContext.Alarms;
            
        CheckAlarmRelevance();
    }
    
    private static async void SaveChanges()
    {
        await _dbContext.SaveChangesAsync();
    } 

    public static void CheckAlarmRelevance()
    {
        var forDeletion = new List<Guid>();

        foreach (var record in AlarmSet)
        {
            if (record.Datetime.CompareTo(DateTime.Now) <= 0)
                forDeletion.Add(new Guid(record.Id));
        }

        bool areThereDeprecatedAlarms = forDeletion.Count != 0;
        
        if (!areThereDeprecatedAlarms) return;
        
        foreach (var id in forDeletion)
        {
            RemoveRecord(id);
        }
        
        SaveChanges();
    }

    public static Alarm AddRecord(string title, DateTime dateTime)
    {
        var alarm = new Alarm(title, dateTime); 
        
        AlarmSet.Add(alarm);
        
        SaveChanges();
        
        return alarm;
    }

    public static Alarm EditRecord(Guid recordId, string title, DateTime dateTime, bool isAlarmEnabled)
    {
        var record = GetRecord(recordId);
        record.Title = title;
        record.Datetime = dateTime;
        record.IsAlarmEnabled = isAlarmEnabled;
        
        SaveChanges();

        return record;
    }

    public static Alarm GetRecord(Guid id)
    {
        var alarm = AlarmSet.Find(id.ToByteArray());

        if (alarm is null)
            throw new ArgumentException("There's no alarms with that GUID");
        
        return alarm;
    }

    public static Alarm RemoveRecord(Guid id)
    {
        var alarm = GetRecord(id);
        
        AlarmSet.Remove(alarm);
        
        SaveChanges();
        
        return alarm;
    }
}