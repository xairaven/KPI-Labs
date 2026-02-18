package Lab4;
import edu.princeton.cs.algs4.In;
import edu.princeton.cs.algs4.Accumulator;
import java.io.FileWriter;
import java.io.IOException;

public class Experiment {
    public static void main(String[] args) {
        final String pathKey = "src/Lab4/OneKeySearch.txt";
        final String pathSubsequence = "src/Lab4/SubsequenceSearch.txt";
        In inArray = new In("src/Lab4/largeW.txt");
        In inArraySorted = new In("src/Lab4/largeWSorted.txt");
        int[] array = inArray.readAllInts();
        int[] arraySorted = inArraySorted.readAllInts();

        final int TIMES = 10000;

        Accumulator linear = new Accumulator();
        Accumulator barrier = new Accumulator();
        Accumulator binary = new Accumulator();
        for (int i = 0; i < TIMES; i++) {
            int key = getRandomNum(0, 1000000);

            long start = System.nanoTime();
            if (Search.linear(key, array) != -1) {
                long end = System.nanoTime() - start;
                linear.addDataValue(end);
            }

            start = System.nanoTime();
            if (Search.barrier(key, array) != -1) {
                long end = System.nanoTime() - start;
                barrier.addDataValue(end);
            }

            start = System.nanoTime();
            if (Search.binary(key, arraySorted) != -1) {
                long end = System.nanoTime() - start;
                binary.addDataValue(end);
            }
        }
        writeTime(pathKey, "AVG Linear: " + linear.mean() + " ns.");
        writeTime(pathKey, "AVG Barrier: " + barrier.mean() + " ns.");
        writeTime(pathKey, "AVG Binary: " + binary.mean() + " ns.");

        Accumulator subsequence = new Accumulator();
        Accumulator rabinKarp = new Accumulator();
        int[] pattern = {946027, 946027, 946028};
        for (int i = 0; i < 500; i++) {
            long start = System.nanoTime();
            if (Search.subsequence(pattern, arraySorted) != -1) {
                long end = System.nanoTime() - start;
                subsequence.addDataValue(end);
            }

            start = System.nanoTime();
            if (Search.RabinKarp(pattern, arraySorted) != -1) {
                long end = System.nanoTime() - start;
                rabinKarp.addDataValue(end);
            }
        }
        writeTime(pathSubsequence, "AVG SubSequence: " + subsequence.mean() + " ns.");
        writeTime(pathSubsequence, "AVG Rabin-Karp Al.: " + rabinKarp.mean() + " ns.");
    }

    public static int getRandomNum(int min, int max) {
        return (int) (Math.random() * (max - min + 1) + min);
    }

    private static void writeTime(String path, String str) {
        try (FileWriter writer = new FileWriter(path, true))
        {
            writer.write(str + "\n");
            writer.flush();
        }
        catch (IOException ex){
            System.out.println(ex.getMessage());
        }
    }
}


