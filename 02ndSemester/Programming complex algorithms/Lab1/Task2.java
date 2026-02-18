package Lab1;
import java.util.Arrays;
import java.util.Scanner;

public class Task2 {
    public static void main() {
        Scanner scan = new Scanner(System.in);
        System.out.println("\n -- Task 2 --");
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
        int[][] origArray = Task1.genQuadArray(N);
        System.out.println("\nStart array\n");
        Task1.showQuadArray(origArray);

        System.out.println("\nResult array (with cyclic shift)\n");
        int[][] arr1 = Task1.copy(origArray);
        arr1 = cyclicShift(arr1, 3);
        Task1.showQuadArray(arr1);
    }

    public static int[][] cyclicShift(int[][] arr, int N) {
        int[][] resArr = new int[arr.length][arr[0].length];
        String first = "";
        for(int i = 0; i < arr.length; i++) {
            first += String.valueOf(i);
        }
        String resultIndex = first.substring(N, first.length()) + first.substring(0, N);
        for(int i = 0; i < resArr.length; i++) {
            resArr[i] = arr[Integer.parseInt(resultIndex.substring(i, i+1))].clone();
        }
       return resArr;
    }
}
