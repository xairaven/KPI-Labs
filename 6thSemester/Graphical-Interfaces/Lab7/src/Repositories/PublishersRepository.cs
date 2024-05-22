using Lab7.Context;
using Lab7.Entities;

namespace Lab7.Repositories;

public class PublishersRepository
{
    private LibraryDbContext _dbContext;

    public PublishersRepository(LibraryDbContext dbContext)
    {
        _dbContext = dbContext;
    }
    
    public Publisher? GetById(int id)
    {
        return _dbContext.Publishers.Find(id);
    }

    public void Create(string name)
    {
        var nameExist = _dbContext.Publishers.Any(e => e.Name.Equals(name));
        
        if (nameExist)
        {
            throw new ArgumentException("Name of publisher company already exists");
        }

        _dbContext.Add(new Publisher { Name = name });
        
        Save();
    }

    public void Delete(int id)
    {
        var element = _dbContext.Publishers.Find(id);

        if (element is null) throw new ArgumentException("There's no publishers with this ID");
        
        _dbContext.Remove(element);
        
        Save();
    }

    public void Update(string initName, string name)
    {
        var initNameExist = _dbContext.Publishers.Any(e => e.Name.Equals(initName));
        var newNameExist = initName.Equals(name) || 
                           _dbContext.Publishers.Any(e => e.Name.Equals(name));
        
        if (!initNameExist || newNameExist)
        {
            throw new ArgumentException("Name of publisher company already exists (or init name is not exist)");
        }

        var initObject = _dbContext.Publishers.SingleOrDefault(e => e.Name.Equals(initName))!;

        initObject.Name = name;
        
        Save();
    }

    public void Save()
    {
        _dbContext.SaveChanges();
    }
}