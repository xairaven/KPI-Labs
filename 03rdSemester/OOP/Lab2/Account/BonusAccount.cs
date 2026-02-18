namespace Lab2;

public class BonusAccount : Account
{
    private readonly int MIN_STREAK = 3;
    public int Bonus => Streak > MIN_STREAK ? Streak * 2 : 0;
    private int Streak { get; set; } = 0;

    public BonusAccount(string username) : base(username)
    {
        Type = AccountType.Bonus;
    }
    
    protected override void WinGame(Game game)
    {
        Streak++;
        Rating += Bonus;
        base.WinGame(game);
    }
    
    protected override void LoseGame(Game game)
    {
        Streak = 0;
        base.LoseGame(game);
    }
}