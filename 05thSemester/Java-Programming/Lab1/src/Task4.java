package Lab1.Task4;

public class Main {
    public static void main(String[] args) {
        if (args.length != 3) throw new IllegalArgumentException("There must be 3 arguments.");

        var result = String.format("%s:%s:%s", args[0], args[1], args[2]);

        System.out.println(result);
    }
}