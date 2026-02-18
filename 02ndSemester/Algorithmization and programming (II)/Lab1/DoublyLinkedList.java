package Lab1;
import java.util.Arrays;
import java.util.Iterator;
import java.util.NoSuchElementException;

@SuppressWarnings("Unchecked")
public class DoublyLinkedList<Item extends Comparable<Item>> implements Iterable<Item> {
    private Node first;
    private Node last;

    private class Node {
        private Item item;
        private Node next;
        private Node previous;
    }

    public DoublyLinkedList() {
        first = null;
        last = null;
    }

    public void addFirst(Item item) {
        Node oldFirst = this.first;
        first = new Node();
        first.item = item;
        first.next = oldFirst;
        first.previous = null;
        if (first.next == null) last = first;
        else oldFirst.previous = first;
    }

    public Item removeFirst() {
        Item item = first.item;
        first = first.next;
        if (first != null) first.previous = null;
        else last = null;
        return item;
    }

    public void addLast(Item item) {
        Node oldLast = last;
        last = new Node();
        last.item = item;
        last.next = null;
        last.previous = oldLast;
        if(last.previous == null) first = last;
    }

    public Item removeLast() {
        Item item = last.item;
        last = last.previous;
        if (last != null) last.next = null;
        else first = null;
        return item;
    }

    public boolean isEmpty() {
        return first == null || last == null;
    }

    public int size() {
        if (isEmpty()) return 0;
        int size = 1;
        Node current = this.first;
        while (current.next != null) {
            size++;
            current = current.next;
        }
        return size;
    }

    public Item get(int N) {
        if (isEmpty()) throw new NoSuchElementException("List is empty");
        if (N < 0) throw new IllegalArgumentException("Index < 0");
        int i = 0;
        Node current = first;
        while (current != null) {
            if (i == N) return current.item;
            current = current.next;
            i++;
        }
        throw new NoSuchElementException("Index is bigger than size of list");
    }

    public Item[] toArray() {
        int size = size();
        //Item[] arr = (Item[]) new Comparable[size];
        Item[] arr = (Item[]) new Comparable[size];
        Node current = first;
        for(int i = 0; i < size; i++) {
            arr[i] = current.item;
            current = current.next;
        }
        return arr;
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

    public Iterator<Item> iterator() { return new ListIterator(); }

    private class ListIterator implements Iterator<Item> {
        private Node current = first;

        @Override
        public boolean hasNext() {
            return current != null;
        }

        @Override
        public Item next() {
            if (!hasNext()) throw new NoSuchElementException();
            Item item = current.item;
            current = current.next;
            return item;
        }

        @Override
        public void remove() {
            throw new UnsupportedOperationException();
        }
    }

    public static void main(String[] args) {
        DoublyLinkedList<Integer> list = new DoublyLinkedList<>();
        System.out.println("Заповнення структури випадковими числами");
        for (int i = 0; i < 20; i++) {
            list.addFirst((int) (Math.random() * 20));
        }

        System.out.println("Друк, використовуючи ітератор");
        for (int i : list) {
            System.out.print(i + " ");
        }

        System.out.println("\n\nВиклик методів remove()");
        for (int i = 0; i < 2; i++) {
            System.out.println("Removed (first): " + list.removeFirst());
            System.out.println("Removed (last): " + list.removeLast());
        }

        System.out.println("\nДрук, використовуючи size + get методи");
        int size = list.size();
        for (int i = 0; i < size; i++) {
            System.out.print(list.get(i) + " ");
        }

        System.out.println("\n\nВиклик методу toArray, друк масиву");
        Comparable[] arr = list.toArray();
        System.out.println(Arrays.toString(arr));

        System.out.println("\nМасив відсортовано?");
        System.out.println(SortSelection.isSorted(arr));

        System.out.println("\nСортування масиву");
        SortInsertion.sort(arr);

        System.out.println("Друк масиву");
        for (Comparable i : arr) {
            System.out.print(i + " ");
        }

        System.out.println("\n\nМасив відсортовано?");
        System.out.println(SortSelection.isSorted(arr));
    }
}
