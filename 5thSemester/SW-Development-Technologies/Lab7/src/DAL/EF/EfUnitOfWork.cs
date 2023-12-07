using DAL.Entities;
using DAL.Repositories.Impl;
using DAL.Repositories.Interfaces;
using DAL.UnitOfWork;
using Microsoft.EntityFrameworkCore;

namespace DAL.EF;

public class EfUnitOfWork : IUnitOfWork
{
    private readonly TransportSystemContext _dbContext;
    private EmployeeRepository? _employeeRepository;
    private OrderRepository? _orderRepository;
    private ShipmentRepository? _shipmentRepository;
    
    public EfUnitOfWork(DbContextOptions options)
    {
        _dbContext = new TransportSystemContext(options);
    }
    
    public IEmployeeRepository Employees
    {
        get
        {
            if (_employeeRepository is null)
                _employeeRepository = new EmployeeRepository(_dbContext);
            return _employeeRepository;
        }
    }
    
    public IOrderRepository Orders
    {
        get
        {
            if (_orderRepository is null)
               _orderRepository = new OrderRepository(_dbContext);
            return _orderRepository;
        }
    }
    
    public IShipmentRepository Shipments
    {
        get
        {
            if (_shipmentRepository is null)
                _shipmentRepository = new ShipmentRepository(_dbContext);
            return _shipmentRepository;
        }
    }
    
    public void Save()
    {
        _dbContext.SaveChanges();
    }
    
    private bool _disposed = false;
    public virtual void Dispose(bool disposing)
    {
        if (_disposed) return;
        if (disposing)
        {
            _dbContext.Dispose();
        }
            
        _disposed = true;
    }
    
    public void Dispose()
    {
        Dispose(true);
        GC.SuppressFinalize(this);
    }
}
