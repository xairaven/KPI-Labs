package Lab4.Utils;

import Lab4.Task4.Entities.Book;
import org.apache.commons.csv.CSVFormat;
import org.apache.commons.csv.CSVRecord;

import java.io.FileReader;
import java.io.IOException;
import java.io.Reader;
import java.util.*;

public class CSVBookReader {
    public static List<Book> parse(String filename) throws IOException {
        Reader in = new FileReader(filename);

        CSVFormat csvFormat = CSVFormat.DEFAULT.builder()
                .setDelimiter(',')
                .setHeader(BookHeaders.class)
                .setSkipHeaderRecord(true)
                .build();

        Iterable<CSVRecord> records = csvFormat.parse(in);

        var books = new LinkedList<Book>();

        for (CSVRecord record : records) {
            String author = record.get(BookHeaders.author);
            String title = record.get(BookHeaders.title);

            var genres = parseStringArray(record.get(BookHeaders.genres));

            books.add(new Book(author, title, genres));
        }

        return books;
    }

    private static List<String> parseStringArray(String str) {
        var textArray = str.substring(1, str.length() - 1);
        var array = textArray.split(", ");

        for (int i = 0; i < array.length; i++) {
            array[i] = array[i].replaceAll("'", "");
        }

        return Arrays.asList(array);
    }
}

enum BookHeaders{
    title, author, genres
}
