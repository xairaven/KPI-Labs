package Lab7;

public class Main {
    public static void main(String[] args) {
        System.out.println("// ------- Hash-Table based SparseMatrix ------- //");
        double[][] matrix = new double[][] {
                {1, 2, 3, 0, 5},
                {0, 0, 0, 9, 0},
                {2, 3, 0, 5, 6},
                {7, 0, 0, 0, 2},
                {3, 0, 5, 6, 0}
        };

        SparseVector[] a = initialiseSparseMatrix(matrix);
        showCompressedMatrix(a);

        System.out.println();

        showMatrix(a);

        System.out.println("\n\n// ------- BST task ------- //");
        BST.main(args);
    }

    private static SparseVector[] initialiseSparseMatrix(double[][] matrix) {
        SparseVector[] a = new SparseVector[matrix.length];
        for (int i = 0; i < a.length; i++) {
            a[i] = new SparseVector(matrix[0].length);
        }

        for (int i = 0; i < matrix.length; i++) {
            for (int j = 0; j < matrix[0].length; j++) {
                a[i].put(j, matrix[i][j]);
            }
        }
        return a;
    }

    private static void showCompressedMatrix(SparseVector[] a) {
        System.out.println("-- Compressed --");
        for (SparseVector i : a) {
            System.out.println(i.toCompressedString());
        }
    }

    private static void showMatrix(SparseVector[] a) {
        System.out.println("-- Extended --");
        for (SparseVector i : a) {
            System.out.println(i);
        }
    }
}
