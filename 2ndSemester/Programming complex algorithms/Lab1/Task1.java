package Lab1;
import java.util.Scanner;

public class Task1 {
    public static void main() {
        Scanner scan = new Scanner(System.in);
        System.out.println("\n -- Task 1 --");
        System.out.println("Choose array size:\n 1. 10x10\n 2. 50x50\n 3. 100x100\n 4. 500x500\n OR any number > 4");
        int chooseN = scan.nextInt();
        int N;
        switch (chooseN) {
            case 1 -> N = 10;
            case 2 -> N = 50;
            case 3 -> N = 100;
            case 4 -> N = 500;
            default -> N = chooseN;
        }
        int[][] origArray = genQuadArray(N);
        System.out.println("\nStart array\n");
        showQuadArray(origArray);

        System.out.println("\nResult array, (all even numbers on the top)\n");
        int[][] arr1 = copy(origArray);
        evenToStartBubble(arr1);
        showQuadArray(arr1);
    }

    public static int[][] genQuadArray(int N) {
        if (N < 2) throw new IllegalArgumentException("Input array size > 2");
        int[][] arr = new int[N][N]; //init
        for (int i = 0; i < N; i++) {
            for (int j = 0; j < N; j++) {
                arr[i][j] = (int) (Math.random() * N); //generating values
            }
        }
        return arr;
    }

    public static void showQuadArray(int[][] arr) {
        for (int i = 0; i < arr.length; i++) {
            System.out.printf("%d:  ", i + 1);
            for (int j = 0; j < arr[0].length; j++) {
                System.out.printf("%d\t", arr[i][j]);
            }
            System.out.print("\n");
        }
    }

    public static int[][] copy(int[][] arr) {
        int[][] newArr = new int[arr.length][arr[0].length];
        for (int i = 0; i < arr.length; i++) {
            for (int j = 0; j < arr[0].length; j++) {
                newArr[i][j] = arr[i][j];
            }
        }
        return newArr;
    }

    public static void evenToStartBubble(int[][] arr) {
        for (int j = 0; j < arr[0].length; j++) {
            boolean evensEmpty = false;
            for (int i = 0; i < arr.length && !evensEmpty; i++) {
                if (arr[i][j] % 2 != 0) {
                    evensEmpty = true;
                    for (int k = i + 1; k < arr.length; k++) {
                        if (arr[k][j] % 2 == 0) {
                            int temp = arr[i][j];
                            arr[i][j] = arr[k][j];
                            arr[k][j] = temp;
                            evensEmpty = false;
                            break;
                        }
                    }
                }
            }
        }
    }
}
