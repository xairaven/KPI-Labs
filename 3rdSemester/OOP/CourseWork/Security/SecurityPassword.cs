using System.Text;
using Shop.Validators;

namespace Shop.Security;

public static class SecurityPassword
{
    public static string Input()
    {
        var sb = new StringBuilder();

        while (true)
        {
            var key = Console.ReadKey(true);
            if (key.Key == ConsoleKey.Enter) break;

            if (key.Key != ConsoleKey.Backspace)
            {
                sb.Append(key.KeyChar);
                Console.Write("*");
            }
            else
            {
                Console.Write("\b \b");
            }
        }
        
        Console.WriteLine();
        Validate(sb.ToString().Trim());

        return MD5.Hash(sb.ToString());
    }

    private static void Validate(string password)
    {
        if (Validator.IsPasswordShort(password)) 
            throw new ArgumentException("Error: Password must have at least 4 symbols");
        if (Validator.IsStringContainsSpaces(password)) 
            throw new ArgumentException("Error: Password can't have gaps.");
    }
}