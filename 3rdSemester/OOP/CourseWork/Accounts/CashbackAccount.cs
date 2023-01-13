using System.Text.Json.Serialization;
using Shop.Accounts.States;

namespace Shop.Accounts;

public class CashbackAccount : Account
{
    public CashbackAccount(string login, string password) : base(login, password)
    {
        Type = AccountType.Cashback;
    }
    
    [Obsolete]
    [JsonConstructor]
    public CashbackAccount(Guid ID, AccountType Type, string Login, string Password)
        : base(ID, Type, Login, Password)
    {}
    
    public override string WelcomeMessage()
    {
        return "Wow, you have a cashback account! Now you will have a 10% cashback!";
    }
}