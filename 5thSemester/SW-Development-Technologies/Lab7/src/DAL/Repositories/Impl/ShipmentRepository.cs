using DAL.EF;
using DAL.Entities;
using DAL.Repositories.Interfaces;

namespace DAL.Repositories.Impl;

public class ShipmentRepository : BaseRepository<Shipment>, IShipmentRepository
{
    internal ShipmentRepository(TransportSystemContext context)
        : base(context)
    {
        
    }
}