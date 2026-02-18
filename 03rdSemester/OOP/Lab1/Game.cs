namespace Lab1;

public class Game
{
    private static uint _idCounter = 1;
    internal static List<Game> Games; // all games

    public GameAccount Winner { get; }
    public GameAccount Loser { get; }
    public uint Score { get; }
    public uint GameId { get; }

    static Game()
    {
        Games = new List<Game>();
    }

    public Game(GameAccount winner, GameAccount loser, uint score)
    {
        Winner = winner;
        Loser = loser;
        Score = score;
        
        GameId = _idCounter++;
    }

    public static void GetHistory()
    {
        Console.ForegroundColor = ConsoleColor.DarkBlue;
        Console.WriteLine($"History of games:");
        Console.ForegroundColor = ConsoleColor.Blue;
        
        if (Games.Count == 0)
        {
            Console.WriteLine("History clear");
        }

        foreach (var game in Games)
        {
            Console.WriteLine($"ID: #{game.GameId}\t Winner: {game.Winner.UserName}\t Loser: {game.Loser.UserName}\t " +
                              $"Bet: {game.Score}");
        }
        
        Console.ResetColor();
    }
}