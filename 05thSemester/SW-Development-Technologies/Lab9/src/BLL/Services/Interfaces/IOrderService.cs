using BLL.DTO;

namespace BLL.Services.Interfaces;

public interface IOrderService
{
    IEnumerable<OrderDto> GetOrders(int page);
}