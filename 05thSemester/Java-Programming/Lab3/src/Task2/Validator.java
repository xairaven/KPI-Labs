package Lab3.Task2;

import java.io.FileNotFoundException;
import java.util.List;

public class Validator {
    private final static int validLength = 12;
    private final static List<String> passwords;

    static  {
        try {
            passwords = FileReader.readPasswordDictionary();
        } catch (FileNotFoundException e) {
            throw new RuntimeException(e);
        }
    }

    public static void check(String password) {
        if (!isPasswordValid(password))
            System.out.println("Пароль не валідний");
        else if (!isPasswordStrong(password))
            System.out.println("Пароль порушує вимоги безпеки");
        else
            System.out.println("Пароль валідний");
    }

    private static boolean isPasswordValid(String password) {
        if (password.contains(" ")) return false;

        return true;
    }

    private static boolean isPasswordStrong(String password) {
        if (password.length() < validLength) return false;

        for (var p : passwords) {
            if (password.equalsIgnoreCase(p)) return false;
        }

        return true;
    }
}
