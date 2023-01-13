using System.Text;
using System.Text.Json.Serialization;
using Shop.Products;
using Shop.Transactions.States;

namespace Shop.Transactions;

public class PurchaseTransaction : Transaction
{
    public List<Product> Products { get; }

    public PurchaseTransaction(Guid accountId, List<Product> products) : base(accountId)
    {
        Type = TransactionType.Purchase;
        Products = new List<Product>();

        foreach (var product in products)
        {
            Products.Add(product);
        }
        
        Amount = (-1) * products.Sum(x => x.Price);
    }
    
    [Obsolete]
    [JsonConstructor]
    public PurchaseTransaction(Guid Id, Guid AccountId, decimal Amount, 
        DateTime Time, TransactionType Type, List<Product> Products) 
        : base(Id, AccountId, Amount, Time, Type)
    {
        this.Products = Products;
    }
    
    public override string ToString()
    {
        var sb = new StringBuilder();

        sb.Append(base.ToString());
        sb.Append($"Products:\n");
        foreach (var product in Products)
        {
            sb.Append($"    {product}\n");
        }
        return sb.ToString();
    }
}