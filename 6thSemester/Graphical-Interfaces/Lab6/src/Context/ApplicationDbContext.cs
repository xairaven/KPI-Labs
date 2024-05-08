using System.Configuration;
using Lab6.Entities;
using Microsoft.EntityFrameworkCore;

namespace Lab6.Context;

public partial class ApplicationDbContext : DbContext
{
    public ApplicationDbContext()
    {
    }

    public ApplicationDbContext(DbContextOptions<ApplicationDbContext> options)
        : base(options)
    {
    }

    public virtual DbSet<Alarm> Alarms { get; set; }

    protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder)
    {
        var connectionString = ConfigurationManager.
            ConnectionStrings["ApplicationDatabase"].ConnectionString;
        
        var dataDirectory = AppDomain.CurrentDomain.GetData("DataDirectory")!.ToString()!;
        if (!string.IsNullOrEmpty(dataDirectory))
        {
            connectionString = connectionString
                .Replace("|DataDirectory|", dataDirectory);
        }
        
        optionsBuilder.UseSqlite(connectionString);
    }
        

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        modelBuilder.Entity<Alarm>(entity =>
        {
            entity.Property(e => e.Id).HasColumnName("id");
            entity.Property(e => e.Datetime)
                .HasColumnType("DATETIME")
                .HasColumnName("datetime");
            entity.Property(e => e.IsAlarmEnabled)
                .HasColumnType("BOOLEAN")
                .HasColumnName("is_alarm_enabled");
            entity.Property(e => e.Title)
                .HasColumnType("NVARCHAR(100)")
                .HasColumnName("title");
        });

        OnModelCreatingPartial(modelBuilder);
    }

    partial void OnModelCreatingPartial(ModelBuilder modelBuilder);
}
