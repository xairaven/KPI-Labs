package Lab4.Task4;

import java.io.IOException;
import java.util.List;

import Lab4.Task4.Entities.Book;
import Lab4.Task4.Entities.Client;
import Lab4.Task4.Entities.Librarian;
import Lab4.Task4.Entities.Library;
import Lab4.Utils.CSVBookReader;

public class Main {
    public static void main(String[] args) {
        Printer printer = new Printer();

        Library library = new Library(parseCSV());

        printer.allBooks(library);

        var title = "1984";
        System.out.printf("Books by title: %s%n", title);
        printer.bookList(library.findBookByTitle(title));

        var author = "Tolkien";
        System.out.printf("Books by author: %s%n", author);
        printer.bookList(library.findBooksByAuthor(author));

        var genre = "Scotland";
        System.out.printf("Books by genre: %s%n", genre);
        printer.bookList(library.findBooksByGenre(genre));


        Librarian alex = new Librarian(library);
        Client vlad = new Client("Vlad", "Karkushevskiy");

        System.out.println("------------------- Books by Arthur Conan Doyle in Library -------------------");
        var doyleBooks = library.findBooksByAuthor("Doyle");
        printer.bookList(doyleBooks);

        System.out.println("------------------- Vlad before borrowing a book -------------------");
        System.out.println(vlad);

        var borrowedBook = alex.checkOutBook(vlad, doyleBooks.get(0));

        System.out.println("------------------- Library after borrowing -------------------");
        doyleBooks = library.findBooksByAuthor("Doyle");
        printer.bookList(doyleBooks);

        System.out.println("------------------- Vlad after borrowing a book -------------------");
        System.out.println(vlad);

        alex.returnBook(vlad, borrowedBook);

        System.out.println("------------------- Vlad after returning a book -------------------");
        System.out.println(vlad);

        System.out.println("------------------- Library after returning -------------------");
        doyleBooks = library.findBooksByAuthor("Doyle");
        printer.bookList(doyleBooks);
    }

    private static List<Book> parseCSV() {
        final String filename = "./src/Lab4/Task4/Data/books.csv";

        try {
            return CSVBookReader.parse(filename);
        } catch (IOException ex) {
            System.out.println(ex.getMessage());
        }

        return null;
    }
}
