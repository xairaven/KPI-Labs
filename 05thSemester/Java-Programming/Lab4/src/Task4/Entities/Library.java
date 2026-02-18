package Lab4.Task4.Entities;

import java.util.LinkedList;
import java.util.List;
import java.util.UUID;

public class Library {
    private final List<Book> books;

    public Library() {
        this.books = new LinkedList<>();
    }

    public Library(List<Book> books) {
        if (books != null) this.books = books;
        else this.books = new LinkedList<>();
    }

    public void addBook(Book book) {
        if (book == null) throw new IllegalArgumentException("Book object has null reference");

        books.add(book);
    }

    public void removeBookById(UUID id) {
        for (Book b : books) {
            if (b.getId() != id) continue;

            books.remove(b);
            return;
        }

        throw new IllegalArgumentException("There's no book with this id");
    }

    public void removeBook(Book book) {
        if (book == null) throw new IllegalArgumentException("Book object has null reference");
        if (!books.contains(book)) throw new IllegalArgumentException("There's no book with this id");

        books.remove(book);
    }

    public List<Book> findBookByTitle(String title) {
        var list = new LinkedList<Book>();

        for (var book : books) {
            if (book.getTitle().toLowerCase().contains(title.toLowerCase())) list.add(book);
        }

        return list;
    }

    public List<Book> findBooksByAuthor(String author) {
        var list = new LinkedList<Book>();

        for (var book : books) {
            if (book.getAuthor().toLowerCase().contains(author.toLowerCase())) list.add(book);
        }

        return list;
    }

    public List<Book> findBooksByGenre(String genre) {
        var list = new LinkedList<Book>();

        for (var book : books) {
            for (var bookGenre : book.getGenres()) {
                if (bookGenre.toLowerCase().contains(genre.toLowerCase())) list.add(book);
            }
        }

        return list;
    }

    public List<Book> listAllBooks() {
        return books;
    }
}
