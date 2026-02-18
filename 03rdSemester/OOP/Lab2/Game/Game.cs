namespace Lab2;

public abstract class Game
{
    private static uint IdCounter;
    public uint GameId { get; }
    
    public Account Winner { get; }
    public int WinnerRating { get; private set; }
    
    public Account Loser { get; }
    public int LoserRating { get; private set; }
    
    public GameType Type { get; protected init; }
    public int Score { get; protected init; }

    public Game(Account player, Account opponent)
    {
        Winner = new Random().Next(0, 2) == 0 
            ? player : opponent;
        Loser = (Winner == opponent)
            ? player : opponent;

        GameId = ++IdCounter;
    }
    
    public static void Play(GameType type, Account player, Account opponent, uint score = 5)
    {
        var factory = new GameFactory();
        Game game = type switch
        {
            GameType.Classic => factory.Classic(player, opponent, score),
            GameType.Training => factory.Training(player, opponent),
            GameType.Lucky => factory.Lucky(player, opponent)
        };
        
        game.Winner.ChangeRating(game);

        game.WinnerRating = game.Winner.Rating;
        game.LoserRating = game.Loser.Rating;
        
        Stats.Add(game);
    }
}