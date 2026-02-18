using DAL.Enums;

namespace DAL.Entities;

public class Shipment
{
    public int Id { get; set; }
    public int OrderId { get; set; }
    public ShipmentStatus Status { get; set; }
    public int DriverId { get; set; }
    public DateTime StartTime { get; set; }
    public DateTime EndTime { get; set; }
}