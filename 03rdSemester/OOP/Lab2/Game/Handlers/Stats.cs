namespace Lab2;

public static class Stats
{
    private static List<Game> History;

    static Stats()
    {
        History = new List<Game>();
    }
    
    public static void Add(Game game)
    {
        if (History.Any(x => x.GameId == game.GameId))
            return;
        History.Add(game);
    }

    public static List<Game> All()
    {
        return History.ToList();
    }

    public static List<Game> ByAccount(Account person)
    {
        return History.Where(x => x.Winner == person || x.Loser == person).ToList();
    }
    
    public static int PreviousRating(Account person, Game game)
    {
        var list = ByAccount(person);
        var id = list.IndexOf(game);
        
        if (id == 0) return Account.InitRating;

        var prevGame = list[id - 1];

        return person == prevGame.Winner ? prevGame.WinnerRating : prevGame.LoserRating;
    }
}