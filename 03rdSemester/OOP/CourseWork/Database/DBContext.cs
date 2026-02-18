using Shop.Accounts;
using Shop.Products;
using Shop.Services;

namespace Shop.Database;

public class DBContext
{
    public List<Account> Users { get; }
    public List<Product> Products { get; }

    public DBContext()
    {
        Users = Json.CustomDeserialize<List<Account>>("Accounts");
        Products = Json.CustomDeserialize<List<Product>>("Products");
        TransactionService.Deserialize("Transactions");
    }
}