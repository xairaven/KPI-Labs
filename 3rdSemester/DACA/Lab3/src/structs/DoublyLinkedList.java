package Lab3.structs;
import Lab3.Hairdressers;

import java.util.Iterator;
import java.util.NoSuchElementException;

public class DoublyLinkedList implements Iterable<Hairdressers>{
    private Node first;
    private Node last;
    private int size;

    private class Node {
        Hairdressers item;
        Node next;
        Node previous;

        private Node(Hairdressers item, Node next, Node previous) {
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

    public Hairdressers get(int N) {
        if (isEmpty()) throw new NoSuchElementException("List is empty");
        if (N - 1 < 0) throw new IllegalArgumentException("Index < 0");
        int i = 0;
        for (Hairdressers item : this) {
            if (i == N - 1) return item;
        }
        throw new NoSuchElementException("Index is bigger than size of list");
    }


    public void pushFirst(Hairdressers item) {
        if (item == null) throw new IllegalArgumentException("You can't push null to list");
        Node oldFirst = first;
        first = new Node(item, oldFirst, null);
        if (oldFirst == null) last = first;
        else oldFirst.previous = first;
        size++;
    }

    public void pushLast(Hairdressers item) {
        if (item == null) throw new IllegalArgumentException("You can't push null to list");
        Node oldLast = last;
        last = new Node(item, null, oldLast);
        if (oldLast == null) first = last;
        else oldLast.next = last;
        size++;
    }

    public void delete(Hairdressers item) {
        if (isEmpty()) throw new NoSuchElementException("List is empty");

        Node current = first;
        for (int i = 0; i < size; i++) {
            if (current == null) {
                return;
            }

            if (current.item.equals(item) && i == 0) {
                popFirst();
                return;
            }

            if (current.item.equals(item) && i == size - 1) {
                popLast();
                return;
            }

            if (current.item.equals(item)) {
                current.previous.next = current.next;
                current.next.previous = current.previous;
                size--;
                return;
            }

            current = current.next;
        }
    }

    public Hairdressers searchByLabel(String label) {
        for (var item : this) {
            if (item.label.equals(label)) {
                return item;
            }
        }
        return null;
    }

    public Hairdressers searchByAddress(String address) {
        for (var item : this) {
            if (item.address.equals(address)) {
                return item;
            }
        }
        return null;
    }

    public Hairdressers searchByWorkers(int workers) {
        for (var item : this) {
            if (item.numberOfWorkers == workers) {
                return item;
            }
        }
        return null;
    }

    public Hairdressers searchByWorktime(String workTime) {
        for (var item : this) {
            if (item.workTime.equals(workTime)) {
                return item;
            }
        }
        return null;
    }

    public Hairdressers popFirst() {
        Hairdressers item = first.item;
        first = first.next;
        if (first != null) first.previous = null;
        else last = null;
        size--;
        return item;
    }

    public Hairdressers popLast() {
        Hairdressers item = last.item;
        last = last.previous;
        if (last != null) last.next = null;
        else first = null;
        size--;
        return item;
    }

    @Override
    public String toString() {
        if (first == null) return "null";
        StringBuilder sb = new StringBuilder();
        Node current = first;
        while(current != null) {
            sb.append(current.item);
            current = current.next;
            if(current != null) sb.append("\n");
        }

        return sb.toString();
    }

    public Iterator<Hairdressers> iterator() { return new ListIterator(); }

    private class ListIterator implements Iterator<Hairdressers> {
        private Node current = first;

        @Override
        public boolean hasNext() {
            return current != null;
        }

        @Override
        public Hairdressers next() {
            if (!hasNext()) throw new NoSuchElementException();
            Hairdressers item = current.item;
            current = current.next;
            return item;
        }

        @Override
        public void remove() {
            throw new UnsupportedOperationException();
        }
    }
}
