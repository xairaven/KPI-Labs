namespace Lab2;

public class GameFactory
{
    public Game Classic(Account player, Account opponent, uint score = 5)
    {
        return new ClassicGame(player, opponent, score);
    }
    
    public Game Training(Account player, Account opponent)
    {
        return new TrainingGame(player, opponent);
    }
    
    public Game Lucky(Account player, Account opponent)
    {
        return new LuckyGame(player, opponent);
    }
}