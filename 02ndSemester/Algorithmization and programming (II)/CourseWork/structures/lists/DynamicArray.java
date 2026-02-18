package ua.kpi.structures.lists;

import java.util.Iterator;
import java.util.NoSuchElementException;

public class DynamicArray<Item> implements Iterable<Item> {
    private Item[] arr;  // array
    private int size;    // fixed size

    public DynamicArray() {
        arr = (Item[]) new Object[2];
        size = 0;
    }

    public boolean isEmpty() {
        return size == 0;
    }

    public int size() {
        return size;
    }

    public Item get(int index) {
        if (index >= size) throw new NoSuchElementException("index is bigger than capacity of array");
        else if (index < 0) throw new ArrayIndexOutOfBoundsException("index must be nonnegative");
        return arr[index];
    }

    public void add(Item item) {
        if (item == null) throw new IllegalArgumentException("can't pull null into array");
        if (size == arr.length) resize(2*arr.length);
        arr[size++] = item;
    }

    public boolean contains(Item item) {
        if (item == null) throw new IllegalArgumentException("array can't contain null");
        for (Item i : this) {
            if (i.equals(item)) return true;
        }
        return false;
    }

    public int indexOf(Item item) {
        if (item == null) throw new IllegalArgumentException("array can't contain null");
        for (int i = 0; i < size; i++) {
            if (arr[i].equals(item)) return i;
        }
        return -1;
    }

    private void resize(int capacity) {
        if (capacity < size) throw new IllegalArgumentException();
        Item[] temp = (Item[]) new Object[capacity];
        for (int i = 0; i < size; i++) {
            temp[i] = arr[i];
        }
        arr = temp;
    }

    @Override
    public String toString() {
        if (arr == null) return "null";
        int iMax = size - 1;
        if (iMax == -1)
            return "[]";
        StringBuilder sb = new StringBuilder();
        sb.append("[");
        for (int i = 0; i < size; i++) {
            sb.append(arr[i]);
            if (i == iMax)
                return sb.append(']').toString();
            sb.append(", ");
        }
        return sb.toString();
    }

    @Override
    public Iterator<Item> iterator() {
        return new ArrayIterator();
    }

    private class ArrayIterator implements Iterator<Item> {
        private int i = 0;

        @Override
        public boolean hasNext() {
            return i < size;
        }

        @Override
        public void remove() {
            throw new UnsupportedOperationException();
        }

        @Override
        public Item next() {
            if (!hasNext()) throw new NoSuchElementException();
            return arr[i++];
        }
    }

    // tests
    public static void main(String[] args) {
        DynamicArray<Integer> arr = new DynamicArray<>();
        for (int i = 11; i < 21; i++) {
            arr.add(i);
        }
        System.out.println(arr);
        System.out.println(arr.contains(null));
        System.out.println(arr.contains(15));
        System.out.println(arr.indexOf(5));
        System.out.println(arr.indexOf(11));
    }
}
