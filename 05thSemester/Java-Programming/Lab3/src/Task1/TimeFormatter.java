package Lab3.Task1;

public class TimeFormatter {
    public static String seconds (long value) {
        if (value < 0) throw new IllegalArgumentException("'Seconds' value must be non-negative.");

        var sb = new StringBuilder();
        sb.append("Час перебору всіх паролів:\n");

        int seconds = (int) (value % 60);
        int minutes = (int) (value % 3600 / 60);
        int hours = (int) (value % 86400 / 3600);
        long days = value / 86400;

        if (days != 0) sb.append(String.format("Днів: %d\n", days));

        if (hours != 0) sb.append(String.format("Годин: %d\n", hours));

        if (minutes!= 0) sb.append(String.format("Хвилин: %d\n", minutes));

        sb.append(String.format("Секунд: %d", seconds));
        return sb.toString();
    }
}
