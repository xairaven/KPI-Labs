package ua.kpi.structures.lists;
import java.util.Iterator;
import java.util.NoSuchElementException;

public class List<Item> implements Iterable<Item> {
    private Node first;
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

    public void push(Item item) {
        if (item == null) throw new IllegalArgumentException("can't push null to the list");
        first = new Node(item, first);
        size++;
    }

    public Item pop() {
        if (isEmpty()) throw new NoSuchElementException("popping elements from empty stack");
        Item item = first.item;
        first = first.next;
        size--;
        return item;
    }

    public Item peek() {
        if (isEmpty()) return null;
        return first.item;
    }

    public void deleteItem(Item item) {
        if ((first.item).equals(item)) {
            first = first.next;
            return;
        }
        for (Node x = first; x.next != null; x = x.next) {
            if (x.next.item.equals(item)) {
                Node further = x.next;
                x.next = further.next;
                return;
            }
        }
        throw new NoSuchElementException("item doesn't in list");
    }

    @Override
    public String toString() {
        StringBuilder s = new StringBuilder();
        for (Item item : this)
            s.append(item + " ");
        return s.toString();
    }

    public Iterator<Item> iterator() {
        return new StackIterator(first);
    }

    private class StackIterator implements Iterator<Item> {
        private Node current;

        public StackIterator(Node first) {
            current = first;
        }

        public boolean hasNext() {
            return current != null;
        }

        public Item next() {
            if (!hasNext()) throw new NoSuchElementException();
            Item item = current.item;
            current = current.next;
            return item;
        }
    }

    // tests
    public static void main(String[] args) {
        List<Integer> list = new List<>();
        for (int i = 0; i < 10; i++) {
            list.push(i);
        }
        for(Integer i: list) {
            System.out.println(i);
        }
    }
}
