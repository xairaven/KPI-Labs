using Shop.Accounts;
using Shop.Database;

namespace Shop.Services;

public class UserService
{
    private readonly DBContext _dbContext;

    public List<Account> Users => _dbContext.Users;
    internal Account CurrentUser { get; set; } = null!;

    public UserService(DBContext dbContext)
    {
        _dbContext = dbContext;
    }

    public void Add(Account user)
    {
        if (!Contains(user.Login))
        {
            _dbContext.Users.Add(user);
        }
    }

    public Account Get(string login)
    {
        return _dbContext.Users.Find(x => x.Login.Equals(login)) ?? throw new InvalidOperationException();
    }

    public bool Contains(string login)
    {
        return _dbContext.Users.Any(x => x.Login.Equals(login));
    }
}