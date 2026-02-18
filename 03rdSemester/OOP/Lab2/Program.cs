namespace Lab2;

class Program
{
    public static void Main(string[] args)
    {
        var players = new List<Account>
        {
            new Account("Alex"),
            new Account("Yaroslav"),
            new BonusAccount("Danylo"),
            new BonusAccount("Liza"),
            new VipAccount("Vladyslav"),
            new VipAccount("Artem")
        };

        var rand = new Random();
        
        for (int i = 0; i < 15; i++)
        {
            var player = players[rand.Next(players.Count)];
            var opponent = players.Where(x => x != player)
                .ToList()[rand.Next(players.Count - 1)];
            
            Game.Play(GameType.Classic, player, opponent, 3);
            Game.Play(GameType.Training, player, opponent);
            Game.Play(GameType.Lucky, player, opponent);
        }
        
        Console.WriteLine("All games history:");
        StatsOutput.Write(Stats.All());

        var randPlayer = players[rand.Next(players.Count)];
        Console.WriteLine($"Game History for {randPlayer.Username}:");
        StatsOutput.Write(Stats.ByAccount(randPlayer));
    }
}