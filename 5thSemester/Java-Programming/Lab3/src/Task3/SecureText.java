package Lab3.Task3;

import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class SecureText {
    public static String mask(String message) {
        return maskCreditCard(maskPhone(message));
    }

    private static String maskPhone(String message) {
        var phonePattern = Pattern.compile("\\b097[-\\s]?\\d{3}[-\\s]?\\d{2}[-\\s]?\\d{2}\\b");
        Matcher matcher = phonePattern.matcher(message);

        String result = message;
        while (matcher.find()) {
            var phone = matcher.group();
            var masked = phone.charAt(0)
                    + phone.replaceAll("\\d", "*").substring(1);

            result = result.replace(phone, masked);
        }

        return result;
    }

    private static String maskCreditCard(String message) {
        var cardPattern = Pattern.compile("\\d{16}|\\d{4}(?:[-\\s]\\d{4}){3}");
        var matcher = cardPattern.matcher(message);

        String result = message;
        while (matcher.find()) {
            var card = matcher.group();
            var masked = card.replaceAll("\\d", "*");

            result = result.replace(card, masked);
        }

        return result;
    }
}
