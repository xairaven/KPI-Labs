using Shop.Accounts.States;
using Shop.Exceptions;
using Shop.Security;
using Shop.Services;
using Shop.UI.Interfaces;
using Shop.Validators;

namespace Shop.UI.Auth;

public class RegistrationController : IUserInterface
{
    private UserService UserService { get; }
    
    public RegistrationController(UserService userService)
    {
        UserService = userService;
    }
    
    public string Message()
    {
        return "Registration";
    }

    public void Action()
    {
        Console.WriteLine("\n-- Registration --");
        string login = LoginInput();
        string password = PasswordInput();
        AccountType type = TypeInput();

        var factory = new AccountFactory(UserService);

        switch (type)
        {
            case AccountType.Basic:
                factory.Basic(login, password);
                break;
            case AccountType.VIP:
                factory.VIP(login, password);
                break;
            case AccountType.Cashback:
                factory.Cashback(login, password);
                break;
        }
        Console.WriteLine($"Success! You are logged in as @{UserService.CurrentUser.Login}\n");
        Console.WriteLine(UserService.CurrentUser.WelcomeMessage());
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
        return SecurityPassword.Input();
    }

    private AccountType TypeInput()
    {
        var types = new List<AccountType>();
        Console.WriteLine("Type:");

        int i = 1;
        foreach (AccountType e in Enum.GetValues(typeof(AccountType)))
        {
            Console.WriteLine($"{i,2}: {e}");
            types.Add(e);
            
            i++;
        }
        Console.WriteLine();

        int choice = Switch(types);

        return types[choice - 1];
    }

    private int Switch(List<AccountType> types)
    {
        int choice;
        while (true)
        {
            Console.Write("Your choice: ");

            int.TryParse(Console.ReadLine(), out choice);

            if (choice < 1 || choice > types.Count)
            {
                throw new ArgumentException("Wrong choice! There isn't variant like that.");
            }
            
            break;
        }

        return choice;
    }
    
    private void ValidateLogin(string login)
    {
        if (Validator.IsLoginShort(login)) 
            throw new ArgumentException("Error: Login must have at least 3 symbols");
        if (Validator.IsStringContainsSpaces(login)) 
            throw new ArgumentException("Error: Login can't have gaps.");
        if (Validator.IsAccountExists(login, UserService)) 
            throw new LoginExistsException("Error: Account already exists, log in with your password");
    }
}