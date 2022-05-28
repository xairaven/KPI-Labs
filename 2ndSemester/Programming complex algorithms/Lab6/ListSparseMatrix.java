package Lab6;
import java.util.Stack;

public class ListSparseMatrix<Item> {
    private int height;
    private int width;
    private Item zeroValue;
    private Node first;

    public class Node {
        private int i;
        private int j;
        private Item value;
        private Node next;

        public Node(Item value, int row, int col, Node next) {
            this.value = value;
            this.i = row;
            this.j = col;
            this.next = next;
        }

        public int[] getCords() {
            return new int[]{i, j};
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
            this.i = i;
            this.j = j;
        }

        public void setValue(Item item) {
            if (item.equals(zeroValue) || item.equals(null)) {
                throw new IllegalArgumentException("given zero value or null to setValue()");
            } else this.value = item;
        }
    }

    public ListSparseMatrix(Item[][] matrix, Item zeroValue) {
        this.height = matrix.length;
        this.width = matrix[0].length;
        this.zeroValue = zeroValue;

        for (int i = 0; i < matrix.length; i++) {
            for (int j = 0; j < matrix[0].length; j++) {
                if (!matrix[i][j].equals(zeroValue)) first = new Node(matrix[i][j], i, j, first);
            }
        }
    }

    public int getWidth() {
        return width;
    }

    public int getHeight() {
        return height;
    }

    @Override
    public String toString() {
        if (first == null) return "null";
        StringBuilder sb = new StringBuilder();
        Node current = first;
        while (current != null) {
            sb.append("(Value = " + current.value);
            sb.append(String.format(", Cords = [%d][%d])\n", current.i, current.j));
            current = current.next;
        }
        return sb.toString();
    }

    public Item[][] toMatrix() {
        Item[][] matrix = (Item[][]) new Object[height][width];
        for (int i = 0; i < matrix.length; i++) {
            for (int j = 0; j < matrix[0].length; j++) {
                matrix[i][j] = zeroValue;
            }
        }

        Node current = first;
        while (current != null) {
            matrix[current.i][current.j] = current.value;
            current = current.next;
        }

        return matrix;
    }

    public Iterable<Node> iterable() {
        Stack<Node> stack = new Stack<>();
        Node current = first;
        while (current != null) {
            stack.push(current);
            current = current.next;
        }
        return stack;
    }
}
