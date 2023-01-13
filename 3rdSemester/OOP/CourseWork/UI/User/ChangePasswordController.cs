using Shop.Services;
using Shop.UI.Interfaces;

namespace Shop.UI.User;

public class ChangePasswordController : IUserInterface
{
    private readonly UserService _userService;

    public ChangePasswordController(UserService userService)
    {
        _userService = userService;
    }
    
    public string Message()
    {
        return "Change password";
    }

    public void Action()
    {
        Console.WriteLine("-- Changing password --");
        _userService.CurrentUser.ChangePassword();
    }
}