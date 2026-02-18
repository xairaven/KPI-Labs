namespace Lab2;

public class TrainingGame : Game
{
    public TrainingGame(Account player, Account opponent) : base(player, opponent)
    {
        Type = GameType.Training;
        Score = 0;
    }
}