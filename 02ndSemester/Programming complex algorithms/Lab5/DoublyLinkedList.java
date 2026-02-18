package Lab5;
import java.util.Iterator;
import java.util.NoSuchElementException;

/**
 * @Author: xairaven
 * THIS IMPLEMENTATION HAVE METHOD task() FOR LAB. IN INDEPENDENT USE THIS METHOD SHOULD BE DELETED
 */

public class DoublyLinkedList<T> implements Iterable<T>{
    private Node first;
    private Node last;
    private int size;

    private class Node {
        T item;
        Node next;
        Node previous;

        private Node(T item, Node next, Node previous) {
            this.item = item;
            this.next = next;
            this.previous = previous;
        }
    }

    public DoublyLinkedList() {
        first = null;
        last = null;
        size = 0;
    }

    public boolean isEmpty() {
        return size == 0;
    }

    public int size() {
        return size;
    }

    public T get(int N) {
        if (isEmpty()) throw new NoSuchElementException("List is empty");
        if (N - 1 < 0) throw new IllegalArgumentException("Index < 0");
        int i = 0;
        for (T item : this) {
            if (i == N - 1) return item;
        }
        throw new NoSuchElementException("Index is bigger than size of list");
    }


    public void pushFirst(T item) {
        if (item == null) throw new IllegalArgumentException("You can't push null to list");
        Node oldFirst = first;
        first = new Node(item, oldFirst, null);
        if (oldFirst == null) last = first;
        else oldFirst.previous = first;
        size++;
    }

    public void pushLast(T item) {
        if (item == null) throw new IllegalArgumentException("You can't push null to list");
        Node oldLast = last;
        last = new Node(item, null, oldLast);
        if (oldLast == null) first = last;
        else oldLast.next = last;
        size++;
    }

    public T popFirst() {
        T item = first.item;
        first = first.next;
        if (first != null) first.previous = null;
        else last = null;
        size--;
        return item;
    }

    public T popLast() {
        T item = last.item;
        last = last.previous;
        if (last != null) last.next = null;
        else first = null;
        size--;
        return item;
    }

    public void task(int index) {
        if (isEmpty()) throw new NoSuchElementException("List is empty");
        if (index < 0 || index > size - 1) throw new IllegalArgumentException("Bad index");
        if (index == 0) popFirst();
        if (index == size - 1) popLast();
        Node current = first;
        for (int i = 0; i < size; i++) {
                if (i == index) {
                    Node previous = current.previous;
                    previous.next = current.next;
                    current = null;
                    return;
                } else {
                    current = current.next;
                }
        }
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

    public Iterator<T> iterator() { return new ListIterator(); }

    private class ListIterator implements Iterator<T> {
        private Node current = first;

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
        System.out.println("\n-- DoublyLinkedList tests --");
        DoublyLinkedList<Integer> DLL = new DoublyLinkedList<>();
        System.out.println("Is list empty? " + DLL.isEmpty());
        System.out.println("Pushing elements");
        for(int i = 1; i <= 20; i++) {
            if (i % 2 == 0) {
                DLL.pushFirst(i);
            } else {
                DLL.pushLast(i);
            }
        }
        System.out.println("Is list empty? " + DLL.isEmpty());
        System.out.println("Initial list with size " + DLL.size() + ":");
        for (Integer i : DLL) {
            System.out.print(i + " ");
        }
        System.out.println();
        for (int i = 0; i < 4; i++) {
            if (i % 2 == 0) {
                System.out.println("Deleting element " + DLL.popLast() + " from end");
            } else {
                System.out.println("Deleting element " + DLL.popFirst() + " from start");
            }
        }
        System.out.println("Result list with size " + DLL.size() + ":");
        for (Integer i : DLL) {
            System.out.print(i + " ");
        }
        System.out.println("\n");
    }
}
