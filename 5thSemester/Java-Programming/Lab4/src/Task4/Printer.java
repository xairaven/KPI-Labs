package Lab4.Task4;

import Lab4.Task4.Entities.Book;
import Lab4.Task4.Entities.Library;

import java.util.List;

public class Printer {
    public void allBooks(Library library) {
        System.out.println("All books available: ");
        for (var book : library.listAllBooks()) {
            System.out.println(book + "\n");
        }
    }

    public void bookList(List<Book> books) {
        for (var book : books) {
            System.out.println(book);
        }
    }
}
