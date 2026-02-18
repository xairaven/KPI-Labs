using System.Text;
using System.Text.Json.Serialization;
using Shop.Transactions.States;

namespace Shop.Transactions;

[JsonDerivedType(typeof(DepositTransaction), typeDiscriminator: "Deposit")]
[JsonDerivedType(typeof(PurchaseTransaction), typeDiscriminator: "Purchase")]
[JsonDerivedType(typeof(CashbackTransaction), typeDiscriminator: "Cashback")]
public abstract class Transaction
{
    public Guid Id { get; }
    public Guid AccountId { get; }
    public decimal Amount { get; protected set; }
    public DateTime Time { get; }
    public TransactionType Type { get; protected set; }

    public Transaction(Guid accountId)
    {
        AccountId = accountId;

        Time = DateTime.Now;
        Id = Guid.NewGuid();
    }

    [Obsolete]
    [JsonConstructor]
    public Transaction(Guid Id, Guid AccountId, decimal Amount, DateTime Time, TransactionType Type)
    {
        this.Id = Id;
        this.AccountId = AccountId;
        this.Amount = Amount;
        this.Time = Time;
        this.Type = Type;
    }

    public override string ToString()
    {
        var sb = new StringBuilder();
        
        sb.Append($"Transaction ID: {Id}\n");
        sb.Append($"Account: {AccountId}\n");
        sb.Append($"Amount: {Amount}\n");
        sb.Append($"Was made on: {Time.ToString("ddd, dd MMM yyy HH':'mm':'ss 'GMT'")}\n");
        sb.Append($"Type of transaction: {Type}\n");

        return sb.ToString();
    }
}