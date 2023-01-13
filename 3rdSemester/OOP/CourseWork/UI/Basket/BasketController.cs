using System.Text;
using Shop.Exceptions;
using Shop.Products;
using Shop.Services;
using Shop.Transactions;
using Shop.UI.Exit;
using Shop.UI.Interfaces;

namespace Shop.UI.Basket;

public class BasketController : IUserInterface
{
    private readonly UserService _userService;
    
    private List<IUserInterface> UIs { get; }

    public BasketController(UserService userService)
    {
        _userService = userService;
        
        UIs = new List<IUserInterface>();

        UIs.Add(new ConfirmPurchaseController(_userService));
        UIs.Add(new DeleteFromBasketController(_userService));
        UIs.Add(new ClearBasketController(_userService));
        UIs.Add(new ExitController());
    }
    
    public string Message()
    {
        return "Basket & Confirm purchase";
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
        Console.WriteLine("\n-- Basket & Confirm purchase --");
        var basket = _userService.CurrentUser.Basket;
        
        if (_userService.CurrentUser.Basket.Count == 0)
        {
            Console.WriteLine("Sorry, basket is clear!");
            throw new ExitException();
        }

        int productNumber = 1;
        foreach (var product in basket)
        {
            Console.WriteLine($"{$"{productNumber}: {product.Name}", -35} {$"Price: {product.Price:F1}", 15}$");
            productNumber++;
        }

        var price = basket.Sum(x => x.Price);
        Console.WriteLine($"Total price: {price}$");
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