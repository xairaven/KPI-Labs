using Shop.Accounts;

namespace Shop.Services;

public class AccountFactory
{
    private UserService _service { get; }
    
    public AccountFactory(UserService service)
    {
        _service = service;
    }

    public void Basic(string login, string password)
    {
        Account account = new Account(login, password);
        RegisterAccount(account);
    }
    
    public void VIP(string login, string password)
    {
        Account account = new VIPAccount(login, password);
        RegisterAccount(account);
    }
    
    public void Cashback(string login, string password)
    {
        Account account = new CashbackAccount(login, password);
        RegisterAccount(account);
    }

    private void RegisterAccount(Account account)
    {
        _service.CurrentUser = account;
        _service.Add(account);
    }
}