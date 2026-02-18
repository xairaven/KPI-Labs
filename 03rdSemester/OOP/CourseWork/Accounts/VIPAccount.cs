using System.Text.Json.Serialization;
using Shop.Accounts.States;

namespace Shop.Accounts;

public class VIPAccount : Account
{
    public VIPAccount(string login, string password) : base(login, password)
    {
        Type = AccountType.VIP;
    }

    [Obsolete]
    [JsonConstructor]
    public VIPAccount(Guid ID, AccountType Type, string Login, string Password)
        : base(ID, Type, Login, Password)
    {}
    
    public override string WelcomeMessage()
    {
        return "Wow, you have a VIP account! Now you can make deposit transactions without commission!";
    }
}