using System.Text.Json.Serialization;

namespace Shop.Products;

public class Product
{
    public Guid ID { get; }
    
    public string Name { get; }
    public ProductCategory Type { get; }
    public decimal Price { get; }
    
    public Product(string name, ProductCategory type, decimal price)
    {
        Name = name;
        Type = type;
        Price = price;
        
        ID = Guid.NewGuid();
    }
    
    [Obsolete]
    [JsonConstructor]
    public Product(Guid ID, string Name, ProductCategory Type, decimal Price)
    {
        this.ID = ID;
        this.Name = Name;
        this.Type = Type;
        this.Price = Price;
    }
    
    public override string ToString()
    {
        return $"[{$"Name: {Name};", -40} {$"Type: {Type};", -25} {$"Price: {Price}$]", -15}";
    }
}