package Lab3.Task2;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;

public class FileReader {
    final static String PATHNAME = "D:\\Programming\\University\\Java-Programming\\src\\Lab3\\Task2\\PasswordDictionary.txt";

    public static List<String> readPasswordDictionary() throws FileNotFoundException {
        var passwords = new ArrayList<String>();

        var scanner = new Scanner(new File(PATHNAME));

        while (scanner.hasNextLine()) {
            passwords.add(scanner.nextLine());
        }

        scanner.close();

        return passwords;
    }
}
