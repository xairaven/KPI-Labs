using DAL.Repositories.Interfaces;
using Microsoft.EntityFrameworkCore;

namespace DAL.Repositories.Impl;

public abstract class BaseRepository<T>(DbContext context) 
    : IRepository<T> where T : class
{
    private readonly DbSet<T> _set = context.Set<T>();

    public void Create(T item)
    {
        _set.Add(item);
    }

    public void Delete(int id)
    {
        var item = Get(id);
        _set.Remove(item);
    }

    public IEnumerable<T> Find(
        Func<T, bool> predicate,
        int pageNumber = 0,
        int pageSize = 10)
    {
        return
            _set.Where(predicate)
                .Skip(pageSize * pageNumber)
                .Take(pageNumber)
                .ToList();
    }

    public T Get(int id)
    {
        return _set.Find(id);
    }

    public IEnumerable<T> GetAll()
    {
        return _set.ToList();
    }

    public void Update(T item)
    {
        context.Entry(item).State = EntityState.Modified;
    }
}