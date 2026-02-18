using Shop.Services;
using Shop.UI.Interfaces;

namespace Shop.UI.Basket;

public class DeleteFromBasketController : IUserInterface
{
    private readonly UserService _userService;

    public DeleteFromBasketController(UserService userService)
    {
        _userService = userService;
    }
    
    public string Message()
    {
        return "Remove item from the basket";
    }

    public void Action()
    {
        var basket = _userService.CurrentUser.Basket;
        
        Console.Write("Enter the numbers of the items you want delete: ");
        var items = (Console.ReadLine() ?? string.Empty).Split(' ', StringSplitOptions.RemoveEmptyEntries);
        Array.Sort(items);
        Array.Reverse(items);
        
        foreach (var item in items)
        {
            if (int.TryParse(item, out var number) && number <= basket.Count)
            {
                basket.RemoveAt(number - 1);
            }
            else
            {
                Console.WriteLine($"There are no product with number {item}");
            }
        }
    }
}