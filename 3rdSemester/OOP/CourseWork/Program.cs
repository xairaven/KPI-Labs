using Shop.UI;

namespace Shop;

public class Program
{
    public static void Main(string[] args)
    {
        var controller = new ManageController();
        controller.Run();
    }
}