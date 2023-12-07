using DAL.Enums;

namespace DAL.Entities;

public class Shipment
{
    public required int Id { get; set; }
    public required int OrderId { get; set; }
    public required ShipmentStatus Status { get; set; }
    public required int DriverId { get; set; }
    public required DateTime StartTime { get; set; }
    public required DateTime EndTime { get; set; }
}