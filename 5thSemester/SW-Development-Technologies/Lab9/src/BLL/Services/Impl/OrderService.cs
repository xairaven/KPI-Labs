using AutoMapper;
using BLL.DTO;
using BLL.Services.Interfaces;
using CCL;
using CCL.Identity;
using DAL.Entities;
using DAL.UnitOfWork;

namespace BLL.Services.Impl;

public class OrderService : IOrderService
{
    private readonly IUnitOfWork _database;
    private readonly int _pageSize = 10;

    public OrderService(IUnitOfWork unitOfWork)
    {
        ArgumentNullException.ThrowIfNull(unitOfWork);

        _database = unitOfWork;
    }

    /// <exception cref="MethodAccessException"></exception>
    public IEnumerable<OrderDto> GetOrders(int pageNumber)
    {
        var user = SecurityContext.GetUser();
        if (user is null || !user.Roles.Contains(Role.Client))
        {
            throw new MethodAccessException();
        }

        var employeeId = user.UserId;
        var ordersEntities =
            _database
                .Orders
                .Find(o => o.ClientId == employeeId, pageNumber, _pageSize);
        var mapper =
            new MapperConfiguration(
                cfg => cfg.CreateMap<Order, OrderDto>()
            ).CreateMapper();
        
        var ordersDto =
            mapper
                .Map<IEnumerable<Order>, List<OrderDto>>(
                    ordersEntities);
        return ordersDto;
    }
}