package Lab3.structs;

import java.io.BufferedWriter;
import java.io.FileWriter;
import java.io.IOException;

public class Writer {
    String file;

    public Writer(String file) {
        this.file = file;
    }

    public void printline(String line) {
        try {
            var writer = new BufferedWriter(new FileWriter(file, true));
            writer.write(line);
            writer.newLine();
            writer.close();
        }
        catch (IOException e) {
            System.out.print(e.getMessage());
        }
    }
}
