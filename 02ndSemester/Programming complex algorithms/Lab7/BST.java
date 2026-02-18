package Lab7;
import edu.princeton.cs.algs4.Queue;
import java.util.NoSuchElementException;

public class BST<Key extends Comparable<Key>, Value> {
    private Node root;

    private class Node {
        private Key key;
        private Value value;
        private Node left, right;
        private int size;

        public Node(Key key, Value value, int size) {
            this.key = key;
            this.value = value;
            this.size = size;
        }
    }

    public boolean isEmpty() {
        return size() == 0;
    }

    public int size() {
        return size(root);
    }

    private int size(Node x) {
        if (x == null) return 0;
        else return x.size;
    }

    public boolean contains(Key key) {
        if (key == null) throw new NullPointerException("argument to contains() is null");
        return get(key) != null;
    }

    public Value get(Key key) {
        return get(root, key);
    }

    private Value get(Node x, Key key) {
        if (x == null) return null;

        int cmp = key.compareTo(x.key);
        if (cmp < 0) return get(x.left, key);
        else if (cmp > 0) return get(x.right, key);
        else return x.value;
    }

    public void put(Key key, Value val) {
        if (key == null) throw new NullPointerException("first argument to put() is null");
        root = put(root, key, val);
    }

    private Node put(Node x, Key key, Value value) {
        if (x == null) return new Node(key, value, 1);
        int cmp = key.compareTo(x.key);
        if (cmp < 0) x.left = put(x.left,  key, value);
        else if (cmp > 0) x.right = put(x.right, key, value);
        else x.value = value;
        x.size = 1 + size(x.left) + size(x.right);
        return x;
    }

    public Key min() {
        if (isEmpty()) throw new NoSuchElementException("called min() with empty symbol table");
        return min(root).key;
    }

    private Node min(Node x) {
        if (x.left == null) return x;
        else                return min(x.left);
    }
    public Key max() {
        if (isEmpty()) throw new NoSuchElementException("called max() with empty symbol table");
        return max(root).key;
    }

    private Node max(Node x) {
        if (x.right == null) return x;
        else                 return max(x.right);
    }

    public Iterable<Key> keys() {
        return keys(min(), max());
    }

    public Iterable<Key> keys(Key lo, Key hi) {
        if (lo == null) throw new NullPointerException("first argument to keys() is null");
        if (hi == null) throw new NullPointerException("second argument to keys() is null");

        Queue<Key> queue = new Queue<Key>();
        keys(root, queue, lo, hi);
        return queue;
    }

    private void keys(Node x, Queue<Key> queue, Key lo, Key hi) {
        if (x == null) return;
        int cmplo = lo.compareTo(x.key);
        int cmphi = hi.compareTo(x.key);
        if (cmplo < 0) keys(x.left, queue, lo, hi);
        if (cmplo <= 0 && cmphi >= 0) queue.enqueue(x.key);
        if (cmphi > 0) keys(x.right, queue, lo, hi);
    }

    public static void main(String[] args) {
        BST<Integer, String> bst = new BST<>();
        bst.put(4, "123");
        bst.put(2, "224");
        bst.put(6, "453");
        bst.put(1, "68");
        bst.put(3, "234");
        bst.put(5, "546");
        bst.put(7, "567");

        System.out.println("BST:");
        for (int x : bst.keys()) {
            System.out.printf("%d:\"%s\"\n", x, bst.get(x));
        }

        System.out.println("\nBST after reversing:");
        bst.reverse(bst.root);
        for (int x : bst.keys()) {
            System.out.printf("%d:\"%s\"\n", x, bst.get(x));
        }

        System.out.println("\nEven task");
        int leftEven = bst.even(bst.root.left);
        int rightEven = bst.even(bst.root.right);

        System.out.printf("Evens in:\nleft subtree - %d\nright subtree - %d\n", leftEven, rightEven);
    }

    @SuppressWarnings("unchecked")
    private Node reverse(Node x) {
        if (x == null) return x;

        var sb = new StringBuilder();
        sb.append(x.value);
        sb.reverse();
        x.value = (Value) sb.toString();
        reverse(x.left);
        reverse(x.right);

        return x;
    }

    private int even(Node x) {
        if (x == null) return 0;

        int even = 0;
        if(Integer.parseInt((String) x.value) % 2 == 0) even++;

        return even + even(x.left) + even(x.right);
    }
}
