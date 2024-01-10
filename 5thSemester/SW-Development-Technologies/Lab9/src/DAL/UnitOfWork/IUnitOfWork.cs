using DAL.Entities;
using DAL.Repositories.Interfaces;

namespace DAL.UnitOfWork;

public interface IUnitOfWork : IDisposable
{
    IEmployeeRepository Employees { get; }
    IOrderRepository Orders { get; }
    IShipmentRepository Shipments { get; }
    void Save();
}