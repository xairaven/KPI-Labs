using System.Text;
using Shop.Exceptions;
using Shop.Services;
using Shop.UI.Interfaces;
using Shop.UI.Exit;

namespace Shop.UI.Auth;

public class AuthController : IUserInterface
{
    private List<IUserInterface> UIs { get; }
    private UserService UserService { get; }

    public AuthController(UserService userService)
    {
        UserService = userService;
        
        UIs = new List<IUserInterface>();
        
        UIs.Add(new LoginController(UserService));
        UIs.Add(new RegistrationController(UserService));
        UIs.Add(new ExitController());
    }
    
    public string Message()
    {
        var sb = new StringBuilder();
        sb.Append("\n-- Log in or Register --\n");

        int menuItem = 1;
        foreach (var controller in UIs)
        {
            sb.Append($"{menuItem,2}: {controller.Message()}");
            if (menuItem != UIs.Count) sb.Append('\n'); 
            
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
                UIs[choice - 1].Action();
            }
            catch (ExitException) { Environment.Exit(0); }
            catch (Exception e)
            {
                Console.WriteLine(e.Message);
                continue;
            }

            break;
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
}