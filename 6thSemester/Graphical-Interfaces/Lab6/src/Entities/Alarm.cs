namespace Lab6.Entities;

public partial class Alarm
{
    public byte[] Id { get; set; } = null!;

    public string Title { get; set; } = null!;

    public bool IsAlarmEnabled { get; set; }

    public DateTime Datetime { get; set; }

    public Alarm()
    {
    }
    
    public Alarm(string title, DateTime dateTime)
    {
        Id = Guid.NewGuid().ToByteArray();
        Title = title;
        IsAlarmEnabled = true;
        Datetime = dateTime;
    }
}
