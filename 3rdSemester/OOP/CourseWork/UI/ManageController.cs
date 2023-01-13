using System.Text;
using Shop.Database;
using Shop.Exceptions;
using Shop.Services;
using Shop.UI.Auth;
using Shop.UI.Basket;
using Shop.UI.Exit;
using Shop.UI.Interfaces;
using Shop.UI.Purchase;
using Shop.UI.User;

namespace Shop.UI;

public class ManageController : IUserInterface
{
    private List<IUserInterface> UIs { get; }
    
    private UserService UserService { get; }
    private ProductService ProductService { get; }

    public ManageController()
    {
        var database = new DBContext();
        UserService = new UserService(database);
        ProductService = new ProductService(database);

        UIs = new List<IUserInterface>();

        UIs.Add(new AddOnBalance(UserService));
        UIs.Add(new ProductListController(UserService, ProductService));
        UIs.Add(new BasketController(UserService));
        UIs.Add(new TransactionHistoryController(UserService));
        UIs.Add(new ChangePasswordController(UserService));
        UIs.Add(new ExitController());
    }
    
    public void Run()
    {
        try
        {
            new AuthController(UserService).Action();
            Action();
        }
        catch (ExitException)
        {
            SerializeAll();
            Console.WriteLine("Goodbye! Hope to see you again!");
            Environment.Exit(0);
        }
    }
    
    public string Message()
    {
        var sb = new StringBuilder();
        sb.Append("\n-- MENU: --");

        sb.Append($"\nCurrent Account: @{UserService.CurrentUser.Login}\n");
        sb.Append($"Type of account: {UserService.CurrentUser.Type}\n");
        sb.Append($"Balance: {UserService.CurrentUser.Balance}$\n\n");

        int menuItem = 1;
        foreach (var controller in UIs)
        {
            sb.Append($"{menuItem,2}: {controller.Message()}\n");

            menuItem++;
        }

        return sb.ToString();
    }

    public void Action()
    {
        while (true)
        {
            try
            {
                int choice = Switch();
                Console.WriteLine();
                UIs[choice - 1].Action();
            }
            catch (ExitException) { throw new ExitException(); }
            catch (Exception e)
            {
                Console.WriteLine(e.Message);
            }
        }
    }
    
    private int Switch()
    {
        int choice;
        while (true)
        {
            Console.WriteLine(Message());
            Console.Write("Your choice: ");

            int.TryParse(Console.ReadLine(), out choice);

            if (choice < 1 || choice > UIs.Count)
            {
                Console.WriteLine("\nWrong choice! There isn't variant like that.");
            } else break;
        }

        return choice;
    }

    private void SerializeAll()
    {
        Json.CustomSerialize("Accounts", UserService.Users);
        Json.CustomSerialize("Products", ProductService.Products);
        Json.CustomSerialize("Transactions", TransactionService.History);
    }
}