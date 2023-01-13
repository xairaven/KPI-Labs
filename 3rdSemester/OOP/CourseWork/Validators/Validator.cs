using Shop.Accounts;
using Shop.Services;

namespace Shop.Validators;

public static class Validator
{
    public static bool IsLoginShort(string data)
    {
        return data.Length < 3;
    }
    
    public static bool IsPasswordShort(string data)
    {
        return data.Length < 4;
    }

    public static bool IsStringContainsSpaces(string data)
    {
        return data.Contains(' ');
    }

    public static bool IsAccountExists(string login, UserService service)
    {
        return service.Contains(login);
    }

    public static bool IsPasswordCorrect(Account account, string password)
    {
        return account.Password.Equals(password);
    }
}