package Lab5;
import java.util.Iterator;
import java.util.NoSuchElementException;

public class Stack<T> implements Iterable<T> {
    private Node head;
    private int size;

    private class Node {
        T item;
        Node next;

        private Node(T item, Node next) {
            this.item = item;
            this.next = next;
        }
    }

    public Stack() {
        head = null;
        size = 0;
    }

    public boolean isEmpty() {
        return size == 0;
    }

    public int size() {
        return size;
    }

    public void push(T item) {
        if (item == null) throw new IllegalArgumentException("You can't push null to stack");
        head = new Node(item, head);
        size++;
    }

    public T pop() {
        if (isEmpty()) throw new NoSuchElementException("Stack is empty");
        T item = head.item;
        head = head.next;
        size--;
        return item;
    }

    public T peek() {
        if (isEmpty()) {
            throw new RuntimeException("Stack underflow");
        }
        return head.item;
    }

    @Override
    public String toString() {
        if (head == null) return "null";
        StringBuilder sb = new StringBuilder();
        sb.append("[");

        Node current = head;
        while(current != null) {
            sb.append(current.item);
            current = current.next;
            if(current != null) sb.append(", ");
        }
        sb.append("]");
        return sb.toString();
    }

    public Iterator<T> iterator() { return new StackIterator(); }

    private class StackIterator implements Iterator<T> {
        private Node current = head;

        @Override
        public boolean hasNext() {
            return current != null;
        }

        @Override
        public T next() {
            if (!hasNext()) throw new NoSuchElementException();
            T item = current.item;
            current = current.next;
            return item;
        }

        @Override
        public void remove() {
            throw new UnsupportedOperationException();
        }
    }

    public static void tests() {
        System.out.println("-- Stack tests --");
        Stack<Integer> stack = new Stack<>();
        System.out.println("Is stack empty? " + stack.isEmpty());
        System.out.println("Pushing elements");
        for(int i = 1; i <= 20; i++) {
            stack.push(i);
        }
        System.out.println("Is stack empty? " + stack.isEmpty());
        System.out.println("Initial stack with size " + stack.size() + ":");
        System.out.println(stack);
        for (int i = 0; i < 3; i++) {
            System.out.println("Deleting element " + stack.pop());
        }
        System.out.println("Result stack with size " + stack.size() + ":");
        for (int i : stack) {
            System.out.print(i + " ");
        }
        System.out.println("\n");
    }
}
