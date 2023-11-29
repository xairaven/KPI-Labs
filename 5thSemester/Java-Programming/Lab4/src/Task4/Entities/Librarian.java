package Lab4.Task4.Entities;

import java.util.LinkedList;
import java.util.List;

public class Librarian {
    private Library library;
    private final List<Client> clients;

    public Librarian() {
        library = new Library();
        clients = new LinkedList<>();
    }

    public Librarian(Library library) {
        this();

        if (library != null) this.library = library;
    }

    public void manageBookAddition(Book book) {
        library.addBook(book);
    }

    public void manageBookRemoval(Book book) {
        library.removeBook(book);
    }

    public Book checkOutBook(Client client, Book book) {
        manageBookRemoval(book);
        var borrowed = client.borrowBook(book);

        if (!clients.contains(client)) clients.add(client);

        return borrowed;
    }

    public Book returnBook(Client client, Book book) {
        var returned = client.returnBook(book);
        manageBookAddition(returned);

        return returned;
    }
}