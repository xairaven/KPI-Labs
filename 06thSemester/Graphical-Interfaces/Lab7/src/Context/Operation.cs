namespace Lab7.Context;

public class Operation<T>
{
    public int Id { get; }
    public string Type { get; }
    private Dictionary<string, T?> _states;

    public Operation(int id, string type)
    {
        Id = id;
        Type = type;

        _states = new Dictionary<string, T?>();
    }

    public T? GetOldState()
    {
        return _states["Old"];
    }
    
    public void SetOldState(T? element)
    {
        _states["Old"] = element;
    }
    
    public T? GetNewState()
    {
        return _states["New"];
    }
    
    public void SetNewState(T? element)
    {
        _states["New"] = element;
    }
}