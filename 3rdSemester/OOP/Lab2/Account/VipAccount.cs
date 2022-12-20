namespace Lab2;

public class VipAccount : Account
{
    public VipAccount(string username) : base(username)
    {
        Type = AccountType.Vip;
    }

    protected override void LoseGame(Game game)
    {
        Rating -= (game.Score / 2);
    }
}