namespace Lab2;

public class LuckyGame : Game
{
    public LuckyGame(Account player, Account opponent) : base(player, opponent)
    {
        Type = GameType.Lucky;
        Score = Math.Min(Winner.Rating, Loser.Rating) - 1;
    }
}