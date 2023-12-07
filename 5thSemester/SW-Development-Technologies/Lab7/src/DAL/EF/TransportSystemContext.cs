using DAL.Entities;
using Microsoft.EntityFrameworkCore;

namespace DAL.EF;

public class TransportSystemContext : DbContext
{
    public DbSet<Employee> Employees { get; set; }
    public DbSet<Order> Orders { get; set; }
    public DbSet<Shipment> Shipments { get; set; }

    public TransportSystemContext(DbContextOptions options)
        : base(options)
    {
    }
}