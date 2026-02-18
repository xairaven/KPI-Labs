package Lab5;
import java.util.Iterator;

public class Queue<T> implements Iterable<T> {
    private Node first;
    private Node last;
    private int size;

    private class Node {
        T item;
        Node next;

        public Node(T item, Node next) {
            this.item = item;
            this.next = next;
        }
    }

    public boolean isEmpty() {
        return size == 0;
    }

    public int size() {
        return size;
    }

    public void enqueue(T item) {
        if (item == null) throw new IllegalArgumentException("You can't enqueue null");
        Node oldLast = last;
        last = new Node(item, null);
        if (isEmpty()) {
            first = last;
        } else {
            oldLast.next = last;
        }
        size++;
    }

    public T dequeue() {
        if (isEmpty()) throw new RuntimeException("Queue underflow");
        T item = first.item;
        Node oldFirst = first;
        first = first.next;
        oldFirst = null;
        size--;
        if (isEmpty()) {
            last = null;
        }
        return item;
    }

    public T peek() {
        if (isEmpty()) {
            throw new RuntimeException("Queue underflow");
        }
        return first.item;
    }

    @Override
    public String toString() {
        if (first == null) return "null";
        StringBuilder sb = new StringBuilder();
        sb.append("[");

        Node current = first;
        while(current != null) {
            sb.append(current.item);
            current = current.next;
            if(current != null) sb.append(", ");
        }
        sb.append("]");
        return sb.toString();
    }

    @Override
    public Iterator<T> iterator() {
        return new QueueIterator();
    }

    private class QueueIterator implements Iterator<T> {
        private Node current = first;

        @Override
        public boolean hasNext() {
            return current != null;
        }

        @Override
        public T next() {
            T item = current.item;
            current = current.next;
            return item;
        }
    }

    public static void tests() {
        System.out.println("-- Queue tests --");
        Queue<Integer> queue = new Queue<>();
        System.out.println("Is queue empty? " + queue.isEmpty());
        System.out.println("Enqueueing elements");
        for(int i = 1; i <= 20; i++) {
            queue.enqueue(i);
        }
        System.out.println("Is queue empty? " + queue.isEmpty());
        System.out.println("Initial queue with size " + queue.size() + ":");
        System.out.println(queue);
        for (int i = 0; i < 3; i++) {
            System.out.println("Dequeueing element " + queue.dequeue());
        }
        System.out.println("Result queue with size " + queue.size() + ":");
        for (int i : queue) {
            System.out.print(i + " ");
        }
        System.out.println("\n");
    }
}
