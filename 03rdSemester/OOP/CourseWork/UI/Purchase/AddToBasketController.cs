using Shop.Services;
using Shop.UI.Interfaces;

namespace Shop.UI.Purchase;

public class AddToBasketController : IUserInterface
{
    private readonly UserService _userService;
    private readonly ProductService _productService;
    
    public AddToBasketController(UserService userService, ProductService productService)
    {
        _userService = userService;
        _productService = productService;
    }
    
    public string Message()
    {
        return "Add to cart";
    }

    public void Action()
    {
        Console.Write("Enter the numbers of the items you want to buy: ");
        var items = (Console.ReadLine() ?? string.Empty).Split(' ', StringSplitOptions.RemoveEmptyEntries);
        foreach (var item in items)
        {
            if (int.TryParse(item, out var number) && number <= _productService.Products.Count)
            {
                _userService.CurrentUser.Basket.Add(_productService.Products[number - 1]);
            }
            else
            {
                Console.WriteLine($"There are no product with number {item}");
            }
        }
    }
}