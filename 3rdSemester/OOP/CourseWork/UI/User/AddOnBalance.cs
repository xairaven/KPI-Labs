using Shop.Services;
using Shop.Transactions;
using Shop.UI.Interfaces;

namespace Shop.UI.User;

public class AddOnBalance : IUserInterface
{
    private UserService UserService { get; }

    public AddOnBalance(UserService userService)
    {
        UserService = userService;
    }
    public string Message()
    {
        return "Transfer money to account";
    }

    public void Action()
    {
        Console.WriteLine("-- Transfer money to account --");
        Console.Write("Amount: ");

        if (!decimal.TryParse(Console.ReadLine(), out var sum))
        {
            throw new ArgumentException("Error: Sum is not number! Please, try again.");
        }
        
        if (sum <= 0) throw new ArgumentException("Error: Value have to be greater than 0!");

        TransactionService.Add(new DepositTransaction(UserService.CurrentUser, sum));
        Console.WriteLine("Success! Transaction was made.");
    }
}