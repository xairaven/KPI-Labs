using Shop.Exceptions;
using Shop.UI.Interfaces;

namespace Shop.UI.Exit;

public class ExitController : IUserInterface
{
    public string Message()
    {
        return "Exit";
    }
    
    public void Action()
    {
        throw new ExitException();
    }
}