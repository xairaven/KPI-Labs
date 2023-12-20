using DAL.EF;
using DAL.Entities;
using DAL.Repositories.Interfaces;

namespace DAL.Repositories.Impl;

public class OrderRepository : BaseRepository<Order>, IOrderRepository
{
    internal OrderRepository(TransportSystemContext context)
        : base(context)
    {
        
    }
}