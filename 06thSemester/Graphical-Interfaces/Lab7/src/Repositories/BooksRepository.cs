using Lab7.Context;
using Lab7.Entities;
using Lab7.Utils;

namespace Lab7.Repositories;

public class BooksRepository
{
    private LibraryDbContext _dbContext;

    public BooksRepository(LibraryDbContext dbContext)
    {
        _dbContext = dbContext;
    }
    
    public Book? GetByIsbn(string isbn)
    {
        return _dbContext.Books.Find(isbn);
    }

    public void Create(string isbn, string title, string authors, string publishers, string publicationYear, bool isUndo=false)
    {
        if (ValidateFields.IsbnExists(_dbContext, isbn))
        {
            throw new ArgumentException("ISBN already exists");
        }
        
        var publishersSet = _dbContext.Publishers;
        
        var publisher = publishersSet.FirstOrDefault(p => p.Name.Equals(publishers));
        Publisher? created = null;
        if (publisher is null)
        {
            var publisherRepo = new PublishersRepository(_dbContext);
            publisherRepo.Create(publishers);
            
            publisher = publishersSet.FirstOrDefault(p => p.Name.Equals(publishers));
            created = publisher;
        }

        var publisherId = publisher!.Id;
        var publicationYearShort = short.Parse(publicationYear);

        var newBook = new Book
        {
            Isbn = isbn,
            Title = title,
            Authors = authors,
            PublisherCode = publisherId,
            PublicationYear = publicationYearShort
        };
        _dbContext.Add(newBook);
        
        Save();

        if (!isUndo)
        {
            OperationContext.Push("Create", null,
                new Book
                {
                    Isbn = newBook.Isbn,
                    Title = newBook.Title,
                    Authors = newBook.Authors,
                    PublisherCode = newBook.PublisherCode,
                    PublicationYear = newBook.PublicationYear
                }, created);
        }
    }
    
    // Create with publishers id
    public void Create(string isbn, string title, string authors, int publishersCode, string publicationYear, bool isUndo=false)
    {
        if (ValidateFields.IsbnExists(_dbContext, isbn))
        {
            throw new ArgumentException("ISBN already exists");
        }
        
        if (!_dbContext.Publishers.Any(p => p.Id == publishersCode))
        {
            throw new ArgumentException("There are not exist any publishers with this id");
        }
        
        var publicationYearShort = short.Parse(publicationYear);

        var newBook = new Book
        {
            Isbn = isbn,
            Title = title,
            Authors = authors,
            PublisherCode = publishersCode,
            PublicationYear = publicationYearShort
        };
        _dbContext.Add(newBook);
        
        Save();

        if (!isUndo)
        {
            OperationContext.Push("Create", null,
                new Book
                {
                    Isbn = newBook.Isbn,
                    Title = newBook.Title,
                    Authors = newBook.Authors,
                    PublisherCode = newBook.PublisherCode,
                    PublicationYear = newBook.PublicationYear
                }, null);
        }
    }

    public void Delete(string isbn, bool isUndo=false)
    {
        var element = _dbContext.Books.Find(isbn);

        if (element is null) throw new ArgumentException("There's no books with this ISBN");
        
        _dbContext.Remove(element);

        if (!isUndo)
        {
            OperationContext.Push("Delete", new Book
            {
                Isbn = element.Isbn,
                Title = element.Title,
                Authors = element.Authors,
                PublisherCode = element.PublisherCode,
                PublicationYear = element.PublicationYear
            }, null, null);
        }
        
        Save();
    }

    public void Update(string isbnInitial, string isbn, string title, string authors, string publishers, string publicationYear, bool isUndo=false)
    {
        var isInitialIsbnExist = ValidateFields.IsbnExists(_dbContext, isbnInitial);
        
        var isIsbnExist = ValidateFields.IsbnExists(_dbContext, isbn) && !isbnInitial.Equals(isbn);
        if (!isInitialIsbnExist || isIsbnExist)
        {
            throw new ArgumentException("""
                                        There are two conditions under which this error can occur:
                                        1. Initial ISBN does not exist;
                                        2. You are trying to change ISBN, and new ISBN already exist.
                                        """);
        }
        
        if (!isbnInitial.Equals(isbn))
        {
            UpdateIsbn(isbnInitial, isbn);
        }
        
        var publishersSet = _dbContext.Publishers;
        
        var publisher = publishersSet.FirstOrDefault(p => p.Name.Equals(publishers));
        Publisher? created = null;
        if (publisher is null)
        {
            var publisherRepo = new PublishersRepository(_dbContext);
            publisherRepo.Create(publishers);
            
            publisher = publishersSet.FirstOrDefault(p => p.Name.Equals(publisher));
            created = publisher;
        }

        var publisherId = publisher!.Id;
        var publicationYearShort = short.Parse(publicationYear);

        var initObject = _dbContext.Books.Find(isbn)!;

        if (!isUndo)
        {
            OperationContext.Push("Edit", new Book
            {
                Isbn = isbnInitial,
                Title = initObject.Title,
                Authors = initObject.Authors,
                PublisherCode = initObject.PublisherCode,
                PublicationYear = initObject.PublicationYear
            }, new Book
            {
                Isbn = isbn,
                Title = title,
                Authors = authors,
                PublisherCode = publisherId,
                PublicationYear = publicationYearShort
            }, created);
        }
        
        initObject.Title = title;
        initObject.Authors = authors;
        initObject.PublisherCode = publisherId;
        initObject.PublicationYear = publicationYearShort;
        
        Save();
    }
    
    // Update with publishers id
    public void Update(string isbnInitial, string isbn, string title, string authors, int publishersCode, string publicationYear, bool isUndo=false)
    {
        var isInitialIsbnExist = ValidateFields.IsbnExists(_dbContext, isbnInitial);
        
        var isIsbnExist = ValidateFields.IsbnExists(_dbContext, isbn) && !isbnInitial.Equals(isbn);
        if (!isInitialIsbnExist || isIsbnExist)
        {
            throw new ArgumentException("""
                                        There are two conditions under which this error can occur:
                                        1. Initial ISBN does not exist;
                                        2. You are trying to change ISBN, and new ISBN already exist.
                                        """);
        }
        
        if (!isbnInitial.Equals(isbn))
        {
            UpdateIsbn(isbnInitial, isbn);
        }

        if (!_dbContext.Publishers.Any(p => p.Id == publishersCode))
        {
            throw new ArgumentException("There are not exist any publishers with this id");
        }
        
        var publicationYearShort = short.Parse(publicationYear);

        var initObject = _dbContext.Books.Find(isbn)!;

        if (!isUndo)
        {
            OperationContext.Push("Edit", new Book
            {
                Isbn = isbnInitial,
                Title = initObject.Title,
                Authors = initObject.Authors,
                PublisherCode = initObject.PublisherCode,
                PublicationYear = initObject.PublicationYear
            }, new Book
            {
                Isbn = isbn,
                Title = title,
                Authors = authors,
                PublisherCode = publishersCode,
                PublicationYear = publicationYearShort
            }, null);
        }
        
        initObject.Title = title;
        initObject.Authors = authors;
        initObject.PublisherCode = publishersCode;
        initObject.PublicationYear = publicationYearShort;
        
        Save();
    }

    private void UpdateIsbn(string isbnInitial, string isbn)
    {
        var initObject = _dbContext.Books.Find(isbnInitial)!;

        var copy = new Book
        {
            Isbn = isbn,
            Title = initObject.Title,
            Authors = initObject.Title,
            PublisherCode = initObject.PublisherCode,
            PublicationYear = initObject.PublicationYear
        };
        
        Delete(isbnInitial);
        Save();
        
        _dbContext.Add(copy);
        
        Save();
    }

    public void Save()
    {
        _dbContext.SaveChanges();
    }
}