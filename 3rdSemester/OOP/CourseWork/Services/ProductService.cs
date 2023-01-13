using Shop.Database;
using Shop.Products;

namespace Shop.Services;

public class ProductService
{
    private readonly DBContext _dbContext;
    
    public List<Product> Products => _dbContext.Products;

    public ProductService(DBContext dbContext)
    {
        _dbContext = dbContext;
        Products.Sort((x,y) =>
            x.Type.CompareTo(y.Type));
    }
}