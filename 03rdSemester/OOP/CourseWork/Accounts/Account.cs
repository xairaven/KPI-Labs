using System.Text.Json.Serialization;
using Shop.Accounts.States;
using Shop.Exceptions;
using Shop.Products;
using Shop.Security;
using Shop.Services;
using Shop.Transactions;
using Shop.Validators;

namespace Shop.Accounts;

[JsonDerivedType(typeof(Account), typeDiscriminator: "Basic")]
[JsonDerivedType(typeof(VIPAccount), typeDiscriminator: "VIP")]
[JsonDerivedType(typeof(CashbackAccount), typeDiscriminator: "Cashback")]
public class Account
{
    public Guid ID { get; }

    public AccountType Type { get; protected set; } = AccountType.Basic;
    
    public string Login { get; }
    
    public string Password { get; private set; }

    internal decimal Balance => History.Sum(x => x.Amount);

    [JsonIgnore]
    public List<Transaction> History => TransactionService.GetByAccountID(ID);

    internal List<Product> Basket { get; }

    public Account(string login, string password)
    {
        Login = login;
        Password = password;

        Basket = new List<Product>();

        ID = Guid.NewGuid();
    }
    
    [Obsolete]
    [JsonConstructor]
    public Account(Guid ID, AccountType Type, string Login, string Password)
    {
        this.ID = ID;
        this.Type = Type;
        this.Login = Login;
        this.Password = Password;

        Basket = new List<Product>();
    }
    
    public virtual string WelcomeMessage()
    {
        return "Congrats, you have basic account! To buy a VIP or cashback account, contact @xairaven!";
    }

    public void ChangePassword()
    {
        Console.Write("Your current password: ");
        string currentPassword = SecurityPassword.Input();
        
        if (!Validator.IsPasswordCorrect(this, currentPassword)) 
            throw new WrongPasswordException("Error: Wrong password. Please, try again.");
        
        Console.Write("New password: ");
        string firstTemp = SecurityPassword.Input();
        
        Console.Write("New password again: ");
        string secondTemp = SecurityPassword.Input();

        if (!firstTemp.Equals(secondTemp)) 
            throw new WrongPasswordException("Error: New passwords don't match");
        
        Password = firstTemp;
        Console.WriteLine("Done, password changed!");
    }
}