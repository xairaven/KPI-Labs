package Lab6;
import java.util.ArrayList;

public class ArraySparseMatrix<Item> {
    private ArrayList<Element> sparseMatrix;
    private final int width;
    private final int height;
    private Item zeroValue;

    public class Element {
        private int[] cords;
        private Item value;

        public Element(Item value, int i, int j) {
            cords = new int[2];

            this.value = value;
            this.cords[0] = i;
            this.cords[1] = j;
        }

        public int getRow() {
            return cords[0];
        }

        public int getCol() {
            return cords[1];
        }

        public Item getValue() {
            return value;
        }

        public void setCords(int i, int j) {
            if (i >= height || i < 0){
                throw new ArrayIndexOutOfBoundsException("given argument i is greater than amount of columns or negative");
            }
            if (j >= width || j < 0){
                throw new ArrayIndexOutOfBoundsException("given argument i is greater than amount of rows or negative");
            }
            cords[0] = i;
            cords[1] = j;
        }

        public void setValue(Item item) {
            if (item.equals(zeroValue) || item.equals(null)) {
                throw new IllegalArgumentException("given zero value or null to setValue()");
            } else this.value = item;
        }

        public boolean equals(Element that) {
            if (this.cords[0] != that.cords[0]) return false;
            else if (this.cords[1] != that.cords[1]) return false;
            else return !(this.value.equals(that.value));
        }
    }

    public ArraySparseMatrix(Item[][] matrix, Item zeroValue) {
        sparseMatrix = new ArrayList<>();
        this.height = matrix.length;
        this.width = matrix[0].length;
        this.zeroValue = zeroValue;

        for (int i = 0; i < matrix.length; i++) {
            for (int j = 0; j < matrix[0].length; j++) {
                if (!matrix[i][j].equals(zeroValue)) sparseMatrix.add(new Element(matrix[i][j], i, j));
            }
        }
    }

    public int getWidth() {
        return width;
    }

    public int getHeight() {
        return height;
    }

    public Item[][] toMatrix() {
        Item[][] matrix = (Item[][]) new Object[height][width];
        for (int i = 0; i < matrix.length; i++) {
            for (int j = 0; j < matrix[0].length; j++) {
                matrix[i][j] = zeroValue;
            }
        }

        for (Element elem : sparseMatrix) {
            matrix[elem.getRow()][elem.getCol()] = elem.getValue();
        }

        return matrix;
    }

    @Override
    public String toString() {
        if (sparseMatrix.isEmpty()) return "null";
        StringBuilder sb = new StringBuilder();
        for (Element elem : sparseMatrix) {
            sb.append("(Value = " + elem.getValue());
            sb.append(String.format(", Cords = [%d][%d])\n", elem.getRow(), elem.getCol()));
        }
        return sb.toString();
    }

    public Iterable<Element> elements() { return sparseMatrix; }
}
