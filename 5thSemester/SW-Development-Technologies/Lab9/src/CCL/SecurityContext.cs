using CCL.Identity;

namespace CCL;

public static class SecurityContext
{
    private static User? _user;
    
    public static User? GetUser()
    {
        return _user;
    }
    
    public static void SetUser(User user)
    {
        _user = user;
    }
}