package Lab1;
import java.util.Iterator;
import java.util.NoSuchElementException;

@SuppressWarnings("Unchecked")
public class DynamicArray<Item extends Comparable<Item>> implements Iterable<Item> {
    private Item[] arr;  // array
    private int size; // fixed size

    /**
     * Constructor
     */
    public DynamicArray() {
        arr = (Item[]) new Comparable[1];
        size = 0;
    }

    /**
     * Is array empty?
     * @return <tt>boolean</tt> true if yes, otherwise returns false
     */
    public boolean isEmpty() {
        return size == 0;
    }

    /**
     * Get size of array
     * @return <tt>Array size</tt>
     */
    public int size() {
        return size;
    }

    /**
     * Get element of array
     * @param index of element
     * @return <tt>Item arr[i]</tt>
     */
    public Item get(int index) {
        if (index >= size) throw new NoSuchElementException("Index is bigger than capacity of array");
        return arr[index];
    }

    /**
     * Add element on the end of array
     * @param item
     */
    public void addLast(Item item) {
        if (item == null) throw new IllegalArgumentException();
        if (size == arr.length) resize(2*arr.length);
        arr[size++] = item;
    }

    /**
     * Returns last item of the array
     * @return <tt>Item lastItem</tt>
     */
    public Item removeLast() {
        if (isEmpty()) throw new NoSuchElementException("Array underflow");
        Item item = arr[size - 1];
        arr[size--] = null;
        if (size > 0 && size == arr.length / 4) resize(arr.length / 2);
        return item;
    }

    /**
     * Selection sort
     * @param arr - DynamicArray
     */
    public static void SortSelection(DynamicArray arr) {
        for (int i = 0; i < arr.size(); i++) {
            int minId = i;
            for (int j = i + 1; j < arr.size(); j++) {
                if (less(arr.get(j), arr.get(minId))) {
                    minId = j;
                }
            }
            arr.exch(i, minId);
        }
    }

    /**
     * Insertion sort
     * @param arr - DynamicArray
     */
    public static void SortInsertion(DynamicArray arr) {
        for (int i = 1; i < arr.size(); i++) {
            for (int j = i; j > 0 && less(arr.get(j), arr.get(j - 1)); j--) {
                arr.exch(j, j - 1);
            }
        }
    }

    /**
     * Is array sorted?
     * @return <tt>boolean true/false</tt>
     */
    public boolean isSorted() {
        for (int i = 1; i < size; i++) {
            if (arr[i].compareTo(arr[i - 1]) < 0) return false;
        }
        return true;
    }

    private void resize(int capacity) {
        if (capacity < size) throw new IllegalArgumentException();
        Item[] temp = (Item[]) new Comparable[capacity];
        for (int i = 0; i < size; i++) {
            temp[i] = arr[i];
        }
        arr = temp;
    }

    private static boolean less(Comparable v, Comparable w) {
        return v.compareTo(w) < 0;
    }

    private void exch(int i, int j) {
        Item t = arr[i];
        arr[i] = arr[j];
        arr[j] = t;
    }

    /**
     * Convert array to string.
     * @return <tt>String arr</tt>
     */
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

    /**
     * Unit-tests
     * @param args
     */
    public static void main(String[] args) {
        DynamicArray<Integer> arr = new DynamicArray<>();

        System.out.println("Заповнення структури випадковими числами");
        for (int i = 0; i < 20; i++) {
            arr.addLast((int) (Math.random() * 20));
            // int temp =  (int) (Math.random() * (85 - 65) + 65);
            // arr.addLast((char) temp);
        }

        System.out.println("Друк, використовуючи ітератор");
        for (int i : arr) {
            System.out.print(i + " ");
        }

        System.out.println("\n\nВиклик методів remove()");
        for (int i = 0; i < 3; i++) {
            System.out.println("Removed: " + arr.removeLast());
        }

        System.out.println("\nДрук, використовуючи size + get методи");
        int size = arr.size();
        for (int i = 0; i < size; i++) {
            System.out.print(arr.get(i) + " ");
        }

        System.out.println("\n\nВиклик методу toArray");
        System.out.println("Відсутній, структура вже є масивом");

        System.out.println("\nДрук масиву");
        System.out.println(arr.toString());

        System.out.printf("\nМассив відсортований: %b\n", arr.isSorted());

        System.out.println("\nСортування масиву");
        SortInsertion(arr);

        System.out.println("Друк масиву");
        for (int i : arr) {
            System.out.print(i + " ");
        }

        System.out.printf("\nМассив відсортований: %b", arr.isSorted());
    }
}
