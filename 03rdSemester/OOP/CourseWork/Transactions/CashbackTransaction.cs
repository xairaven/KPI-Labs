using System.Text;
using System.Text.Json.Serialization;
using Shop.Accounts;
using Shop.Accounts.States;
using Shop.Transactions.States;

namespace Shop.Transactions;

public class CashbackTransaction : Transaction
{
    private readonly List<AccountType> Permission = new()
    {
        AccountType.Cashback,    
    };

    public double Percent { get; } = 0.1;
    
    public CashbackTransaction(Account account, decimal amount) : base(account.ID)
    {
        Type = TransactionType.Cashback;

        if (!Permission.Contains(account.Type))
        {
            Amount = 0;
            Console.WriteLine("You don't have cashback function!");
            return;
        }
        
        Amount = amount * (1 - (decimal) Percent);
    }
    
    [Obsolete]
    [JsonConstructor]
    public CashbackTransaction(Guid Id, Guid AccountId, decimal Amount, 
        DateTime Time, TransactionType Type, double Percent) 
        : base(Id, AccountId, Amount, Time, Type)
    {
        this.Percent = Percent;
    }
    
    public override string ToString()
    {
        var sb = new StringBuilder();

        sb.Append(base.ToString());
        sb.Append($"Percent: {Percent * 100}%");

        return sb.ToString();
    }
}