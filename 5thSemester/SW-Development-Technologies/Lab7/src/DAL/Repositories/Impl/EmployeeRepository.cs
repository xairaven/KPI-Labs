using DAL.EF;
using DAL.Entities;
using DAL.Repositories.Interfaces;

namespace DAL.Repositories.Impl;

public class EmployeeRepository : BaseRepository<Employee>, IEmployeeRepository
{
    internal EmployeeRepository(TransportSystemContext context)
        : base(context)
    {
        
    }
}