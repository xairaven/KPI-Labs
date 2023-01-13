using Shop.Accounts;
using Shop.Exceptions;
using Shop.Security;
using Shop.Services;
using Shop.UI.Interfaces;
using Shop.Validators;

namespace Shop.UI.Auth;

public class LoginController : IUserInterface
{
    private UserService UserService { get; }

    public LoginController(UserService userService)
    {
        UserService = userService;
    }
    
    public string Message()
    {
        return "Log in";
    }

    public void Action()
    {
        Console.WriteLine("\n-- Log in --");
        string login = LoginInput();
        
        Account candidate = UserService.Get(login);
        
        string password = PasswordInput();
        
        ValidatePassword(candidate, password);
        Console.WriteLine();
        
        UserService.CurrentUser = candidate;
        Console.WriteLine($"Success! You are logged in as @{UserService.CurrentUser.Login}\n");
    }
    
    private string LoginInput()
    {
        Console.Write($"{"Login:", -10}");
        
        var login = (Console.ReadLine() ?? string.Empty).Trim();
        ValidateLogin(login);

        return login;
    }
    
    private string PasswordInput()
    {
        Console.Write($"{"Password:", -10}");
        
        string password = SecurityPassword.Input();

        return password;
    }

    private void ValidateLogin(string login)
    {
        if (Validator.IsLoginShort(login)) 
            throw new ArgumentException("Error: Login must have at least 3 symbols");
        if (Validator.IsStringContainsSpaces(login)) 
            throw new ArgumentException("Error: Login can't have gaps.");
        if (!Validator.IsAccountExists(login, UserService)) 
            throw new LoginExistsException("Error: Account does not exist");
    }

    private void ValidatePassword(Account candidate, string password)
    {
        if (!Validator.IsPasswordCorrect(candidate, password))
        {
            throw new WrongPasswordException("Error: Wrong password. Please, try again.");
        }
    }
}