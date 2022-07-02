package ua.kpi.structures.lists;
import java.util.Iterator;
import java.util.NoSuchElementException;

public class Queue<Item> implements Iterable<Item> {
    private Node first;
    private Node last;
    private int size;

    private class Node {
        private Item item;
        private Node next;

        public Node (Item item, Node next) {
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

    public void enqueue(Item item) {
        if (item == null) throw new IllegalArgumentException("item can't be null");
        Node oldLast = last;
        last = new Node(item, null);
        if (isEmpty()) first = last;
        else oldLast.next = last;
        size++;
    }

    public Item dequeue() {
        if (isEmpty()) throw new NoSuchElementException("can't dequeue in empty queue");
        Item item = first.item;
        first = first.next;
        size--;
        if (isEmpty()) last = null;
        return item;
    }

    public Item peek() {
        if (isEmpty()) throw new NoSuchElementException("queue underflow");
        return first.item;
    }

    @Override
    public String toString() {
        StringBuilder s = new StringBuilder();
        for (Item item : this)
            s.append(item + " ");
        return s.toString();
    }

    @Override
    public Iterator<Item> iterator() {
        return new QueueIterator(first);
    }

    private class QueueIterator implements Iterator<Item> {
        private Node current;

        public QueueIterator(Node first) {
            current = first;
        }

        @Override
        public boolean hasNext() {
            return current != null;
        }

        @Override
        public Item next() {
            Item item = current.item;
            current = current.next;
            return item;
        }
    }

    // tests
    public static void main(String[] args) {
        Queue<Integer> queue = new Queue<>();
        for (int i = 0; i < 10; i++) {
            if (i == 5) queue.dequeue();
            if (i == 8) queue.dequeue();
            queue.enqueue(i);
        }
        for(Integer i: queue) {
            System.out.println(i);
        }
    }
}
