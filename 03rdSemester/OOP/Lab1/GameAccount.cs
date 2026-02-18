namespace Lab1;

public class GameAccount
{
    public string UserName { get; }
    private List<Game> gameList;
    
    public uint CurrentRating
    {
        get
        {
            uint rating = 10; // init rating

            foreach (var game in gameList)
            {
                if (game.Winner == this)
                {
                    rating += game.Score;
                }
                else if ((int) rating - (int) game.Score < 1) // overflow fixed
                {
                    rating = 1;
                }
                else rating -= game.Score;
            }
            return rating;
        }
    }

    public GameAccount(string userName)
    {
        UserName = userName;

        gameList = new List<Game>();
    }

    public void WinGame(GameAccount opponent, uint score)
    {
        var game = new Game(this, opponent, score);

        gameList.Add(game);
        opponent.gameList.Add(game); // opponent loses
        
        Game.Games.Add(game);
    }
    
    public void LoseGame(GameAccount opponent, uint score)
    {
        var game = new Game(opponent, this, score);
        
        gameList.Add(game);
        opponent.gameList.Add(game); // this.account loses
        
        Game.Games.Add(game);
    }

    public void GetStats()
    {
        Console.ForegroundColor = ConsoleColor.DarkGreen;
        Console.WriteLine($"STATS OF {UserName}");
        Console.ForegroundColor = ConsoleColor.Green;
        
        Console.WriteLine($"Current rating: {CurrentRating}");
        Console.WriteLine($"Games played: {gameList.Count}");
        Console.WriteLine($"Wins: {gameList.Count(i => i.Winner == this)}");
        Console.WriteLine($"Defeats: {gameList.Count(i => i.Loser  == this)}");
        
        Console.ResetColor();
    }

    public void GetHistory()
    {
        Console.ForegroundColor = ConsoleColor.DarkRed;
        Console.WriteLine($"History of {UserName} games:");
        Console.ForegroundColor = ConsoleColor.Red;
        
        if (gameList.Count == 0)
        {
            Console.WriteLine("History clear");
        }

        foreach (var game in gameList)
        {
            Console.WriteLine($"ID: {game.GameId}\t Winner: {game.Winner.UserName}\t Loser: {game.Loser.UserName}\t " +
                              $"Bet: {game.Score}");
        }
        
        Console.ResetColor();
    }
}

