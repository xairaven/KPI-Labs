using System.Text;
using Shop.Exceptions;
using Shop.Products;
using Shop.Services;
using Shop.UI.Exit;
using Shop.UI.Interfaces;

namespace Shop.UI.Purchase;

public class ProductListController : IUserInterface
{
    private readonly ProductService _productService;
    
    private List<IUserInterface> UIs { get; }
    
    public ProductListController(UserService userService, ProductService productService)
    {
        _productService = productService;
        
        UIs = new List<IUserInterface>();

        UIs.Add(new AddToBasketController(userService, _productService));
        UIs.Add(new ExitController());
    }
    public string Message()
    {
        return "View products and add to cart";
    }

    public void Action()
    {
        while (true)
        {
            try
            {
                int choice = Switch();
                UIs[choice - 1].Action();
            }
            catch (ExitException) { break; }
            catch (Exception e)
            {
                Console.WriteLine(e.Message);
            }
        }
    }

    private void ShowList()
    {
        Console.WriteLine("\n-- View products and add to cart --");
        if (_productService.Products.Count == 0)
        {
            Console.WriteLine("Sorry, there are no goods!");
            return;
        }
        
        int productNumber = 1;

        var category = ProductCategory.Other;
        foreach (var product in _productService.Products)
        {
            if (product.Type != category)
            {
                category = product.Type;
                Console.WriteLine($"- {category}");
            }
            
            Console.WriteLine($"  {$"{productNumber}: {product.Name}", -35} {$"Price: {product.Price:F1}", 15}$");
            productNumber++;
        }
    }
    
    private int Switch()
    {
        int choice;
        while (true)
        {
            ShowList();
            Console.WriteLine(Menu());
            Console.Write("Your choice: ");

            int.TryParse(Console.ReadLine(), out choice);

            if (choice < 1 || choice > UIs.Count)
            {
                Console.WriteLine("\nWrong choice! There isn't variant like that.");
            } else break;
        }

        return choice;
    }

    private string Menu()
    {
        var sb = new StringBuilder();
        sb.Append("\n");

        int menuItem = 1;
        foreach (var controller in UIs)
        {
            sb.Append($"{menuItem,2}: {controller.Message()}");
            if (menuItem != UIs.Count) sb.Append('\n'); 
            
            menuItem++;
        }

        return sb.ToString();
    }
}