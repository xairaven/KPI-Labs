package ua.kpi.parser;
import java.io.BufferedInputStream;
import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.util.ArrayList;
import java.util.InputMismatchException;
import java.util.Locale;
import java.util.NoSuchElementException;
import java.util.Scanner;
import java.util.regex.Pattern;

public final class In {
    private static final String CHARSET_NAME = "UTF-8";
    private static final Locale LOCALE = Locale.US;
    private static final Pattern EVERYTHING_PATTERN = Pattern.compile("\\A");
    private static final Pattern WHITESPACE_PATTERN = Pattern.compile("\\p{javaWhitespace}+");
    private Scanner scanner;

    public In(File file) {
        if (file == null) throw new NullPointerException("argument to In() is null");
        try {
            FileInputStream fis = new FileInputStream(file);
            scanner = new Scanner(new BufferedInputStream(fis), CHARSET_NAME);
            scanner.useLocale(LOCALE);
        } catch (IOException exception) {
            throw new IllegalArgumentException("Could not open " + file);
        }
    }

    public In(String name) {
        if (name == null) throw new NullPointerException("argument to In() is null");
        try {
            File file = new File(name);
            if (file.exists()) {
                FileInputStream fis = new FileInputStream(file);
                scanner = new Scanner(new BufferedInputStream(fis), CHARSET_NAME);
                scanner.useLocale(LOCALE);
            }
        } catch (IOException exception) {
            throw new IllegalArgumentException("Could not open " + name);
        }
    }

    public In(Scanner scanner) {
        if (scanner == null) throw new NullPointerException("argument is null");
        this.scanner = scanner;
    }

    public boolean exists()  {
        return scanner != null;
    }

    public boolean isEmpty() {
        return !scanner.hasNext();
    }

    public boolean hasNextLine() {
        return scanner.hasNextLine();
    }

    public String readLine() {
        String line;
        try {
            line = scanner.nextLine();
        } catch (NoSuchElementException e) {
            line = null;
        }
        return line;
    }

    public String readAll() {
        if (!scanner.hasNextLine()) return "";
        return scanner.useDelimiter(EVERYTHING_PATTERN).next();
    }

    public String readString() {
        return scanner.next();
    }

    public int readInt() {
        return scanner.nextInt();
    }

    public double readDouble() {
        return scanner.nextDouble();
    }

    public float readFloat() {
        return scanner.nextFloat();
    }

    public long readLong() {
        return scanner.nextLong();
    }

    public short readShort() {
        return scanner.nextShort();
    }

    public byte readByte() {
        return scanner.nextByte();
    }

    public boolean readBoolean() {
        String s = readString();
        if (s.equalsIgnoreCase("true")) return true;
        if (s.equalsIgnoreCase("false")) return false;
        if (s.equals("1")) return true;
        if (s.equals("0")) return false;
        throw new InputMismatchException();
    }

    public String[] readAllStrings() {
        String[] tokens = WHITESPACE_PATTERN.split(readAll());
        if (tokens.length == 0 || tokens[0].length() > 0) return tokens;
        String[] decapitokens = new String[tokens.length-1];
        for (int i = 0; i < tokens.length-1; i++) decapitokens[i] = tokens[i+1];
        return decapitokens;
    }

    public String[] readAllLines() {
        ArrayList<String> lines = new ArrayList<>();
        while (hasNextLine()) {
            lines.add(readLine());
        }
        return lines.toArray(new String[0]);
    }


    public int[] readAllInts() {
        String[] fields = readAllStrings();
        int[] values = new int[fields.length];
        for (int i = 0; i < fields.length; i++) values[i] = Integer.parseInt(fields[i]);
        return values;
    }

    public long[] readAllLongs() {
        String[] fields = readAllStrings();
        long[] values = new long[fields.length];
        for (int i = 0; i < fields.length; i++) values[i] = Long.parseLong(fields[i]);
        return values;
    }

    public double[] readAllDoubles() {
        String[] fields = readAllStrings();
        double[] values = new double[fields.length];
        for (int i = 0; i < fields.length; i++)
            values[i] = Double.parseDouble(fields[i]);
        return values;
    }

    public void close() {
        scanner.close();
    }
}
