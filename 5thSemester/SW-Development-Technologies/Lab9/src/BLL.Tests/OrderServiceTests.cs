using BLL.DTO;
using BLL.Services.Impl;
using BLL.Services.Interfaces;
using CCL;
using CCL.Identity;
using DAL.UnitOfWork;
using Moq;

namespace BLL.Tests;

public class OrderServiceTests
{
    [Fact]
    public void Ctor_InputNull_ThrowArgumentNullException()
    {
        // Arrange
        IUnitOfWork? nullUnitOfWork = null;

        // Act
        var actualServiceFunc = () => new OrderService(nullUnitOfWork);

        // Assert
        Assert.Throws<ArgumentNullException>(actualServiceFunc);
    }

    [Fact]
    public void GetOrders_UserIsClient_ThrowMethodAccessException()
    {
        // Arrange
        var user = new User(1, Role.Client);
        SecurityContext.SetUser(user);

        var mockUnitOfWork = new Mock<IUnitOfWork>();
        IOrderService orderService = new OrderService(mockUnitOfWork.Object);

        // Act
        var actualGetOrdersFunc = () => orderService.GetOrders(0);
        var exception = Record.Exception(actualGetOrdersFunc);

        // Assert
        Assert.IsNotType<MethodAccessException>(exception);
    }

    [Fact]
    public void GetOrders_OrdersFromDAL_CorrectMappingToOrderDTO()
    {
        // Arrange
        User user = new User(1, Role.Client, Role.Coordinator);
        SecurityContext.SetUser(user);

        var expectedOrderDto = new OrderDto
        {
            Id = 1,
            ClientId = 1,
            Title = "Apples | 5kg",
            Origin = "ATB",
            Destination = "Alex Kovalov"
        };
        
        var orderServiceFake = new OrderServiceFake(expectedOrderDto);
        var actualService = orderServiceFake.Get();

        // Act
        var actualOrderDto = actualService.GetOrders(0).First();

        // Assert
        Assert.True(
            actualOrderDto.Id == expectedOrderDto.Id
            && actualOrderDto.ClientId == expectedOrderDto.ClientId
            && actualOrderDto.Title.Equals(expectedOrderDto.Title)
            && actualOrderDto.Origin.Equals(expectedOrderDto.Origin)
            && actualOrderDto.Destination.Equals(expectedOrderDto.Destination)
        );
    }
}