using BLL.DTO;
using BLL.Services.Impl;
using BLL.Services.Interfaces;
using DAL.Entities;
using DAL.Repositories.Interfaces;
using DAL.UnitOfWork;
using Moq;

namespace BLL.Tests;

public class OrderServiceFake
{
    private OrderDto _orderDto;

    public OrderServiceFake(OrderDto orderDto)
    {
        _orderDto = orderDto;
    }
    
    internal IOrderService Get()
    {
        var mockContext = new Mock<IUnitOfWork>();

        var expectedOrder = new Order
        {
            Id = _orderDto.Id,
            ClientId = _orderDto.ClientId,
            Title = _orderDto.Title,
            Origin = _orderDto.Origin,
            Destination = _orderDto.Destination
        };

        var mockDbSet = new Mock<IOrderRepository>();
        mockDbSet
            .Setup(repo =>
                repo.Find(
                    It.IsAny<Func<Order,bool>>(),
                    It.IsAny<int>(),
                    It.IsAny<int>()))
            .Returns(
                new List<Order> { expectedOrder }
            );
        
        mockContext
            .Setup(context =>
                context.Orders)
            .Returns(mockDbSet.Object);
        
        IOrderService orderService = new OrderService(mockContext.Object);
        return orderService;
    }

}