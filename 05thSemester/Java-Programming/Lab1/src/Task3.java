package Lab1.Task3;

public class Main {
    public static void main(String[] args) {
        var year = 2023;
        var weirdNumber = 5.1613;
        var programmingLang = "Java";
        var isNumberBigger = 13 < 79;

        var result = String.format("%d\n%f\n%s\n%b",
                year,
                weirdNumber,
                programmingLang,
                isNumberBigger);

        System.out.println(result);
    }
}
