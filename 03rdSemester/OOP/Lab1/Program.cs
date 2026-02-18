namespace Lab1;

public class Program
{
    public static void Main(string[] args)
    {
        var Alex = new GameAccount("Alex");
        var Vasyl = new GameAccount("Vasyl");
        
        var Vlad = new GameAccount("Vlad");
        var Artem = new GameAccount("Artem");
        
        Alex.WinGame(Vasyl, 5);
        
        Alex.GetStats(); // user history
        Console.WriteLine("\n");
        
        Alex.GetHistory();
        Console.WriteLine("\n");
        
        Vasyl.GetStats();
        Console.WriteLine("\n");

        for (int i = 1; i < 6; i++)
        {
            Artem.WinGame(Vlad, (uint) i); // sixth iter - possible overflow
        }
        
        Game.GetHistory(); // full history
        Console.WriteLine("\n");
        
        Artem.GetStats();
        Console.WriteLine("\n");
        
        Vlad.GetStats();
    }
}