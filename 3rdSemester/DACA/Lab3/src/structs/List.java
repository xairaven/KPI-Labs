package Lab3.structs;

import Lab3.Hairdressers;

import java.util.Iterator;
import java.util.NoSuchElementException;

public class List implements Iterable<Hairdressers>{
    private Node head;  // head of linked list
    private int size;   // size of linked list

    private class Node {
        Hairdressers item;
        Node next;

        private Node(Hairdressers item, Node next) {
            this.item = item;
            this.next = next;
        }
    }

    public List() {
        head = null;
        size = 0;
    }

    public boolean isEmpty() {
        return size == 0;
    }

    public int size() {
        return size;
    }

    public void push(Hairdressers item) {
        if (item == null) throw new IllegalArgumentException("You can't push null to list");
        head = new Node(item, head);
        size++;
    }

    public Hairdressers pop() {
        if (isEmpty()) throw new NoSuchElementException("List is empty");
        Hairdressers item = head.item;
        head = head.next;
        size--;
        return item;
    }

    public void delete(Hairdressers item) {
        if (isEmpty()) throw new NoSuchElementException("List is empty");

        if ((head.item).equals(item)) {
            head = head.next;
            size--;
            return;
        }

        for (Node current = head.next, previous = head; current != null; current = current.next, previous = previous.next) {
            if (current.item == item) {
                previous.next = current.next;
                size--;
            }
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

    public Hairdressers peek() {
        if (isEmpty()) {
            return null;
        }
        return head.item;
    }

    @Override
    public String toString() {
        if (head == null) return "";
        var sb = new StringBuilder();

        Node current = head;
        for (int i = size - 1; i >= 0; i--) {
            sb.append(current.item);
            current = current.next;

            if (i != 0) {
                sb.append("\n");
            }
        }
        return sb.toString();
    }

    public Iterator<Hairdressers> iterator() { return new ListIterator(); }

    private class ListIterator implements Iterator<Hairdressers> {
        private Node current = head;

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
