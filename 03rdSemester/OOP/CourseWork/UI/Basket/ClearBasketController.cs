using Shop.Exceptions;
using Shop.Services;
using Shop.UI.Interfaces;

namespace Shop.UI.Basket;

public class ClearBasketController : IUserInterface
{
    private readonly UserService _userService;

    public ClearBasketController(UserService userService)
    {
        _userService = userService;
    }
    
    public string Message()
    {
        return "Remove all goods from the basket";
    }

    public void Action()
    {
        _userService.CurrentUser.Basket.Clear();
        Console.WriteLine("Done, basket is empty!");
        throw new ExitException();
    }
}