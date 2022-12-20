namespace Lab2;

public class StatsOutput
{
    public static void Write(List<Game> list)
    {
        Console.ForegroundColor = ConsoleColor.Blue;
        Console.WriteLine(
              "*----------------------------------------------------------------------------------------------------*\n"
            + "| ID  |  GameType  | Score | Winner   <->     Type |   Rating   | Loser    <->     Type |   Rating   |\n"
            + "*----------------------------------------------------------------------------------------------------*"
        );
        foreach (var game in list)
        {
            Console.ForegroundColor = game.Type switch
            {
                GameType.Classic => ConsoleColor.Green,
                GameType.Training => ConsoleColor.White,
                GameType.Lucky => ConsoleColor.Red
            };

            Console.WriteLine(
                $"| {game.GameId, -3} | {game.Type, -10} | {game.Score, -5} " +
                $"| {game.Winner.Username, -10} {game.Winner.Type, 10} " +
                $"| {Stats.PreviousRating(game.Winner, game), -3} -> {game.WinnerRating, 3} " +
                $"| {game.Loser.Username, -10} {game.Loser.Type, 10} " +
                $"| {Stats.PreviousRating(game.Loser, game), -3} -> {game.LoserRating, 3} |");
        }

        Console.WriteLine(
            "*----------------------------------------------------------------------------------------------------*"
        );
        Console.ForegroundColor = ConsoleColor.Gray;
    }
}