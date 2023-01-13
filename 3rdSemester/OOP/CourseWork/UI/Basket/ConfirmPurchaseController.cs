using Shop.Accounts.States;
using Shop.Exceptions;
using Shop.Services;
using Shop.Transactions;
using Shop.UI.Interfaces;
using Transaction = System.Transactions.Transaction;

namespace Shop.UI.Basket;

public class ConfirmPurchaseController : IUserInterface
{
    private readonly UserService _userService;

    public ConfirmPurchaseController(UserService userService)
    {
        _userService = userService;
    }
    
    public string Message()
    {
        return "Confirm purchase";
    }

    public void Action()
    {
        var basket = _userService.CurrentUser.Basket;

        var price = basket.Sum(x => x.Price);
        if (price > _userService.CurrentUser.Balance)
        {
            Console.WriteLine($"Sorry, you have not enough money! " +
                              $"Please, top up your account for {price - _userService.CurrentUser.Balance}$");
            return;
        }

        var purchased = basket.ToList();
        basket.Clear();
        
        TransactionService.Add(new PurchaseTransaction(_userService.CurrentUser.ID, purchased));

        if (_userService.CurrentUser.Type == AccountType.Cashback)
        {
            TransactionService.Add(new CashbackTransaction(
                _userService.CurrentUser, price ));
        } 
        
        throw new ExitException();
    }
}