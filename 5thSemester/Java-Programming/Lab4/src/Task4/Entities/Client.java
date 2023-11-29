package Lab4.Task4.Entities;

import java.util.LinkedList;
import java.util.List;
import java.util.UUID;

public class Client {
    private final UUID id;
    private final String firstName;
    private final String lastName;

    private final List<Book> borrowedBooks;

    public Client(String firstName, String lastName) {
        if (firstName.trim().isEmpty() || lastName.trim().isEmpty()) {
            throw new IllegalArgumentException("First or last name is empty.");
        }

        id = UUID.randomUUID();
        this.firstName = firstName;
        this.lastName = lastName;

        this.borrowedBooks = new LinkedList<>();
    }

    public UUID getId() {
        return id;
    }

    public String getFirstName() {
        return firstName;
    }

    public String getLastName() {
        return lastName;
    }

    public List<Book> getBorrowedBooks() {
        return borrowedBooks;
    }

    public Book borrowBook(Book book) {
        if (book == null) throw new IllegalArgumentException("Book reference is null.");

        borrowedBooks.add(book);
        return book;
    }

    public Book returnBookById(UUID id) {
        Book desiredBook = null;

        for (var book : borrowedBooks) {
            if (book.getId() != id) continue;

            desiredBook = book;
        }

        if (desiredBook == null) throw new IllegalArgumentException("There's no book with that ID");

        borrowedBooks.remove(desiredBook);

        return desiredBook;
    }

    public Book returnBook(Book book) {
        if (book == null) throw new IllegalArgumentException("Book reference is null.");
        if (!borrowedBooks.contains(book)) throw new IllegalArgumentException("Client did not borrow this book.");

        borrowedBooks.remove(book);
        return book;
    }

    @Override
    public String toString() {
        var sb = new StringBuilder();

        sb.append(String.format("Client ID: %s%n", id));
        sb.append(String.format("First name: %s%n", firstName));
        sb.append(String.format("Last name: %s%n", lastName));
        sb.append(String.format("Borrowed books: (%d)%n", borrowedBooks.size()));

        for (int i = 0; i < borrowedBooks.size(); i++) {
            var book = borrowedBooks.get(i);
            sb.append(String.format("\t%d. \"%s\" | %s%n", i+1, book.getTitle(), book.getAuthor()));
        }

        return sb.toString();
    }
}
