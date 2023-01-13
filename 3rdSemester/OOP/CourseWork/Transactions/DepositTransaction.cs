using System.Text;
using System.Text.Json.Serialization;
using Shop.Accounts;
using Shop.Accounts.States;
using Shop.Transactions.States;

namespace Shop.Transactions;

public class DepositTransaction : Transaction
{
    private readonly List<AccountType> NoCommission = new()
    {
        AccountType.VIP,    
    };
    
    public double Commission { get; }
    public DepositTransaction(Account account, decimal amount) : base(account.ID)
    {
        Type = TransactionType.Deposit;

        if (!NoCommission.Contains(account.Type))
        {
            Commission = 0.1;
            Console.WriteLine($"Commission is {Commission * 100}%! Please, buy VIP subscription.");
        }
        
        Amount = amount * (1 - (decimal) Commission);
    }
    
    [Obsolete]
    [JsonConstructor]
    public DepositTransaction(Guid Id, Guid AccountId, decimal Amount, 
        DateTime Time, TransactionType Type, double Commission) 
        : base(Id, AccountId, Amount, Time, Type)
    {
        this.Commission = Commission;
    }
    
    public override string ToString()
    {
        var sb = new StringBuilder();

        sb.Append(base.ToString());
        sb.Append($"Commission: {Commission * 100}%");

        return sb.ToString();
    }
}