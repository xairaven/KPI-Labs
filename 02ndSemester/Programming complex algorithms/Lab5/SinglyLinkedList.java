package Lab5;
import java.util.Arrays;
import java.util.Iterator;
import java.util.NoSuchElementException;

/**
 * @Author: xairaven
 * THIS IMPLEMENTATION HAVE METHOD task() FOR LAB. IN INDEPENDENT USE THIS METHOD SHOULD BE DELETED
 */

public class SinglyLinkedList<T> implements Iterable<T>{
    private Node head;  // head of linked list
    private int size;   // size of linked list

    private class Node {
        T item;
        Node next;

        private Node(T item, Node next) {
            this.item = item;
            this.next = next;
        }
    }

    public SinglyLinkedList() {
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
        if (item == null) throw new IllegalArgumentException("You can't push null to list");
        head = new Node(item, head);
        size++;
    }

    public T pop() {
        if (isEmpty()) throw new NoSuchElementException("List is empty");
        T item = head.item;
        head = head.next;
        size--;
        return item;
    }

    public void task(int index) {
        if (isEmpty()) throw new NoSuchElementException("List is empty");
        if (index < 0 || index > size - 1) throw new IllegalArgumentException("Bad index");
        if (index == 0) pop();
        Node previous = null;
        Node current = head;
        if (index == size - 1) {
            for (int i = 0; i < size; i++) {
                if (i == index) {
                    current = null;
                    if (previous != null) {
                        previous.next = null;
                    }
                    return;
                } else {
                    previous = current;
                    current = current.next;
                }
            }
        } else {
            for (int i = 0; i < size; i++) {
                if (i == index) {
                    Node next = current.next;
                    current = null;
                    previous.next = next;
                    T temp = next.item;
                    next.item = previous.item;
                    previous.item = temp;
                    size--;
                    return;
                } else {
                    previous = current;
                    current = current.next;
                }
            }
        }
    }

    public T peek() {
        if (isEmpty()) {
            return null;
        }
        return head.item;
    }

    @Override
    public String toString() {
        T[] arr = (T[]) new Object[size];
        Node current = head;
        for (int i = size - 1; i >= 0; i--) {
            arr[i] = current.item;
            current = current.next;
        }
        return Arrays.toString(arr);
    }

    public Iterator<T> iterator() { return new ListIterator(); }

    private class ListIterator implements Iterator<T> {
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
        System.out.println("-- SinglyLinkedList tests --");
        SinglyLinkedList<Integer> SLL = new SinglyLinkedList<>();
        System.out.println("Is list empty? " + SLL.isEmpty());
        System.out.println("Pushing elements");
        for(int i = 1; i <= 20; i++) {
            SLL.push(i);
        }
        System.out.println("Is list empty? " + SLL.isEmpty());
        System.out.println("Initial list with size " + SLL.size() + ":");
        System.out.println(SLL);
        for (int i = 0; i < 3; i++) {
            System.out.println("Deleting element " + SLL.pop() + " from end");
        }
        System.out.println("Result list with size " + SLL.size() + ":");
        System.out.println(SLL);
    }
}
