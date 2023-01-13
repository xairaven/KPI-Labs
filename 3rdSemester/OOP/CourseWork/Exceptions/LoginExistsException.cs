namespace Shop.Exceptions;

public class LoginExistsException : Exception
{
    public LoginExistsException()
    {
    }

    public LoginExistsException(string message)
        : base(message)
    {
    }

    public LoginExistsException(string message, Exception inner)
        : base(message, inner)
    {
    }
}