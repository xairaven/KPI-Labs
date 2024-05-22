using System.Windows.Forms;
using Lab7.Entities;
using Lab7.Repositories;
using MessageBox = System.Windows.Forms.MessageBox;

namespace Lab7.Context;

public static class OperationContext
{
    public static int Length => BooksStack.Count;

    private static short _idCounter;
    
    private static Stack<Operation<Book>> BooksStack { get; set; }
    private static Stack<Operation<Publisher>> PublishersStack { get; set; }

    static OperationContext()
    {
        BooksStack = new Stack<Operation<Book>>();
        PublishersStack = new Stack<Operation<Publisher>>();
    }

    public static void Pop()
    {
        var operationBook = BooksStack.Pop();
        BookOperationUndo(operationBook);
        
        if (PublishersStack.Count != 0 && operationBook.Id == PublishersStack.Peek().Id)
        {
            PublisherOperationUndo(PublishersStack.Pop());
        }
    }
    
    public static void Push(string type, Book? oldBook, Book? newBook, Publisher? newPublisher)
    {
        var id = _idCounter++;

        var operationBook = new Operation<Book>(id, type);
        operationBook.SetOldState(oldBook);
        operationBook.SetNewState(newBook);
        BooksStack.Push(operationBook);

        if (newPublisher is not null)
        {
            var operationPublisher = new Operation<Publisher>(id, "Create");
            operationPublisher.SetNewState(newPublisher);
            PublishersStack.Push(operationPublisher);
        }
    }

    private static void BookOperationUndo(Operation<Book> operation)
    {
        switch (operation.Type)
        {
            case "Create": 
                BookUndoCreate(operation);
                break;
            case "Edit":
                BookUndoEdit(operation);
                break;
            case "Delete":
                BookUndoDelete(operation);
                break;
        }
    }

    private static void BookUndoCreate(Operation<Book> operation)
    {
        var newBook = operation.GetNewState();

        using var dbContext = new LibraryDbContext();
        try
        {
            new BooksRepository(dbContext).Delete(newBook!.Isbn, isUndo: true);
        }
        catch (Exception exception)
        {
            MessageBox.Show(text: exception.Message,
                caption: "Error!",
                buttons: MessageBoxButtons.OK,
                icon: MessageBoxIcon.Error);
        }
    }
    
    private static void BookUndoEdit(Operation<Book> operation)
    {
        var oldBook = operation.GetOldState()!;
        var newBook = operation.GetNewState()!;

        using var dbContext = new LibraryDbContext();
        try
        {
            new BooksRepository(dbContext).Update(newBook.Isbn, oldBook.Isbn, 
                oldBook.Title, oldBook.Authors, oldBook.PublisherCode, oldBook.PublicationYear.ToString()!,
                isUndo: true);
        }
        catch (Exception exception)
        {
            MessageBox.Show(text: exception.Message,
                caption: "Error!",
                buttons: MessageBoxButtons.OK,
                icon: MessageBoxIcon.Error);
        }
    }
    
    private static void BookUndoDelete(Operation<Book> operation)
    {
        var oldBook = operation.GetOldState()!;

        using var dbContext = new LibraryDbContext();
        try
        {
            new BooksRepository(dbContext).Create(oldBook.Isbn, oldBook.Title, 
                oldBook.Authors, oldBook.PublisherCode, oldBook.PublicationYear.ToString()!, isUndo: true);
        }
        catch (Exception exception)
        {
            MessageBox.Show(text: exception.Message,
                caption: "Error!",
                buttons: MessageBoxButtons.OK,
                icon: MessageBoxIcon.Error);
        }
    }
    
    private static void PublisherOperationUndo(Operation<Publisher> operation)
    {
        PublisherUndoCreate(operation);
    }
    
    private static void PublisherUndoCreate(Operation<Publisher> operation)
    {
        var newPublisher = operation.GetNewState();

        using var dbContext = new LibraryDbContext();
        try
        {
            new PublishersRepository(dbContext).Delete(newPublisher!.Id);
        }
        catch (Exception exception)
        {
            MessageBox.Show(text: exception.Message,
                caption: "Error!",
                buttons: MessageBoxButtons.OK,
                icon: MessageBoxIcon.Error);
        }
    }
}