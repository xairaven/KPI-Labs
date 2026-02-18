package Lab1.misc;

public class Matrix {
    // Генерація матриці
    public static int[][] gen(int N) {
        int[][] arr = new int[N][N];

        for (int i = 0; i < arr.length; i++) {
            for (int j = 0; j < arr[0].length; j++) {
                arr[i][j] = (int) (Math.random() * 20);
            }
        }

        return arr;
    }

    // Виведення матриці на екран
    public static void show(int[][] arr) {
        for (int[] row : arr) {
            for (int elem : row) {
                System.out.printf("%d\t", elem);
            }
            System.out.println();
        }
    }

    // Копіювання матриці
    public static int[][] copy(int[][] arr) {
        int[][] temp = new int[arr.length][arr[0].length];

        for (int i = 0; i < arr.length; i++) {
            for (int j = 0; j < arr[0].length; j++) {
                temp[i][j] = arr[i][j];
            }
        }

        return temp;
    }

    // Транспонування матриці
    public static int[][] transpose(int[][] arr) {
        int[][] temp = new int[arr[0].length][arr.length];
        for(int i = 0; i < arr[0].length; i++) {
            for(int j = 0; j < arr.length; j++) {
                temp[i][j] = arr[j][i];
            }
        }
        return temp;
    }

    // Весь процес сортування, використовуючи вказівник на функцію
    public static int[][] sort(FunctionPointer pointer, int[][] initMatrix, Timer timer) {
        int[][] tempMatrix = copy(initMatrix);

        tempMatrix = transpose(tempMatrix);

        timer.start();
        for (int[] matrix : tempMatrix) {
            pointer.methodSignature(matrix);
        }
        timer.stop();

        tempMatrix = transpose(tempMatrix);

        return tempMatrix;
    }
}
