package Lab1.misc;
public class Timer {
    private long start = 0;
    private long time = 0;

    public Timer() {
        time = 0;
    }

    public void start() {
        start = System.nanoTime();
    }

    public void stop() {
        long end = System.nanoTime();
        time = end - start;

        start = 0;
    }

    public long getTime() {
        return time;
    }
}
