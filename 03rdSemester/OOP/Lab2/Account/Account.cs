namespace Lab2;

public class Account
{
    public const int InitRating = 5;
    private const int MinRating = 1;

    public AccountType Type { get; protected init; } = AccountType.Basic;
    public string Username { get; }

    private int _rating = InitRating;
    public int Rating
    {
        get => _rating;
        protected set => _rating = value < MinRating ? MinRating : value;
    }
    public int GamesCount => Stats.ByAccount(this).Count;

    public Account(string username)
    {
        Username = username;
    }

    public void ChangeRating(Game game)
    {
        game.Winner.WinGame(game);
        game.Loser.LoseGame(game);
    }
    
    protected virtual void WinGame(Game game)
    {
        Rating += game.Score;
    }

    protected virtual void LoseGame(Game game)
    {
        Rating -= game.Score;
    }

    public List<Game> MyGames()
    {
        return Stats.ByAccount(this);
    }
}