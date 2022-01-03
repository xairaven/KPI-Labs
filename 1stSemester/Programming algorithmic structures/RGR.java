package com.company;
import java.util.Scanner;

//Ковальов Олександр, ТР-12
public class RgrTr12Kova {
    public static void main(String[] args) {
        System.out.println("-- Завдання 1 -- ");
        double[][] matrix = {
                {-8, -6,   2,  8,  -7,  45.9},
                { 5,  3, -10,  5,  -3, -176.5},
                {-6, -3, -10, 10, -10, -109.5},
                { 8, -8,   2, -6,  -3, -24.7},
                { 2,  3,   7, -4,   1,  66.1},
        };
        double[][] trianMatrix = task11(matrix);
        double[] vectorRes = task12(trianMatrix);
        System.out.println("\nПочаткова матриця:");
        task13(matrix);
        System.out.println("\nТрикутникова матриця:");
        task13(trianMatrix);
        System.out.println("\nВектор результатів:");
        task13(vectorRes);

        System.out.println("\n\n-- Завдання 2 --");
        Scanner scan = new Scanner(System.in);
        int num;
        for (int i = 0; i < 3; i++) {
            num = scan.nextInt();
            System.out.printf("Кількість дільників числа %d дорівнює %d\n", num, task2(num));
        }
    }

    public static double[][] task11(double[][] mat) {
        //Створюємо нову матрицю
        double[][] trianMatrix = new double[mat.length][mat[0].length];
        for (int i = 0; i < mat.length; i++) {
            for (int j = 0; j < mat[0].length; j++) {
                trianMatrix[i][j] = mat[i][j];
            }
        }
        //Прямий хід методу Гауса
        double temp = 0;
        for(int mainRows = 1; mainRows < trianMatrix.length; mainRows++) {
            int row = mainRows;
            while (row < trianMatrix.length) {
                temp = trianMatrix[row][mainRows - 1] / trianMatrix[mainRows - 1][mainRows-1];
                for (int col = 0; col < trianMatrix.length + 1; col++) {
                    trianMatrix[row][col] = trianMatrix[row][col] - temp * trianMatrix[mainRows - 1][col];
                }
                row++;
            }
        }
        return trianMatrix;
    }

    public static double[] task12(double[][] mat) {
        //Зворотній хід методу Гаусв
        double[] x = new double[mat[0].length - 1];
        for (int i = mat.length - 1; i >= 0; i--) {
            x[i] = mat[i][mat.length]/mat[i][i];
            int pos = mat.length - 1;
            while(pos > i) {
                x[i] = x[i] - mat[i][pos] * x[pos] / mat[i][i];
                pos--;
            }
        }
        return x;
    }

    public static void task13(double[][] mat) {
        //перевантажений метод для виведення матриць
        for (int i = 0; i < mat.length; i++) {
            for (int j = 0; j < mat[0].length; j++) {
                if (mat[i][j] % 1 == 0) {
                    System.out.printf("%.0f\t\t", mat[i][j]);
                } else {
                    System.out.printf("%.3f  ", mat[i][j]);
                }
            }
            System.out.printf("\n");
        }
    }

    public static void task13(double[] mat) {
        //перевантажений метод для виведення вектора результатів
        for (int i = 0; i < mat.length; i++) {
            if (mat[i] % 1 == 0) {
                System.out.printf("%.0f  ", mat[i]);
            } else {
                System.out.printf("%.3f  ", mat[i]);
            }
        }
    }

    public static int task2(int num) {
        int k = 0;
        for (int i = 1; i <= num / 2; i++) {
            if (num % i == 0) k++;
        }
        return k + 1;
    }
}
