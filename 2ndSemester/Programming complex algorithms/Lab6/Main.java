package Lab6;
import java.util.Arrays;

public class Main {
    public static void main(String[] args) {
        Integer[][] matrix = new Integer[][] {
                {1, 2, 3, 0, 5},
                {0, 0, 0, 9, 0},
                {2, 3, 0, 5, 6},
                {7, 0, 0, 0, 2},
                {3, 0, 5, 6, 0}
        };
        System.out.println("-- LAB 6, Kovalyov O. TP-12, Var. 21 --");

        System.out.println("\n--- ARRAY-STYLE SPARSE MATRIX ---\n");
        ArraySparseMatrix<Integer> arraySparseMatrix = new ArraySparseMatrix<>(matrix, 0);
        System.out.println("Initial (array-style) compressed matrix");
        System.out.println(arraySparseMatrix);
        System.out.println("Initial (array-style) extended matrix");
        Object[][] extendedArrayMatrix = arraySparseMatrix.toMatrix();
        System.out.println(Arrays.deepToString(extendedArrayMatrix));

        // task
        int width = arraySparseMatrix.getWidth();
        for (ArraySparseMatrix<Integer>.Element e : arraySparseMatrix.elements()) {
            e.setCords(e.getRow(), (width-1-e.getCol()));
        }

        System.out.println("\nResult (array-style) compressed matrix");
        System.out.println(arraySparseMatrix);
        System.out.println("Result (array-style) extended matrix");
        extendedArrayMatrix = arraySparseMatrix.toMatrix();
        System.out.println(Arrays.deepToString(extendedArrayMatrix));

        System.out.println("\n--- LIST-STYLE SPARSE MATRIX ---\n");
        ListSparseMatrix<Integer> listSparseMatrix = new ListSparseMatrix<>(matrix, 0);
        System.out.println("Initial (list-style) compressed matrix");
        System.out.println(listSparseMatrix);
        System.out.println("Initial (list-style) extended matrix");
        Object[][] extendedListMatrix = listSparseMatrix.toMatrix();
        System.out.println(Arrays.deepToString(extendedListMatrix));

        // task
        width = listSparseMatrix.getWidth();
        for (ListSparseMatrix<Integer>.Node n : listSparseMatrix.iterable()) {
            n.setCords(n.getCords()[0], (width-1-n.getCords()[1]));
        }

        System.out.println("\nResult (list-style) compressed matrix");
        System.out.println(listSparseMatrix);
        System.out.println("Result (list-style) extended matrix");
        extendedListMatrix = listSparseMatrix.toMatrix();
        System.out.println(Arrays.deepToString(extendedListMatrix));
    }
}
