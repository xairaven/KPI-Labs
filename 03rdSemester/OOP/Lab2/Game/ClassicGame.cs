namespace Lab2;

public class ClassicGame : Game
{
    public ClassicGame(Account player, Account opponent, uint score) : base(player, opponent)
    {
        Type = GameType.Classic;
        Score = (int) score;
    }
}