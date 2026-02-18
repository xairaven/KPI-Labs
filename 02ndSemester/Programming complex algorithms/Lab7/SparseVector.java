package Lab7;
import java.util.Hashtable;

public class SparseVector {
    private final int d;                          // dimension
    private final Hashtable<Integer, Double> st;  // the vector, represented by index-value pairs

    public SparseVector(int d) {
        this.d = d;
        this.st = new Hashtable<>();
    }

    public void put(int i, double value) {
        if (i < 0 || i >= d) throw new IndexOutOfBoundsException("Illegal index");
        if (value == 0.0) st.remove(i);
        else st.put(i, value);
    }

    public double get(int i) {
        if (i < 0 || i >= d) throw new IndexOutOfBoundsException("Illegal index");
        if (st.contains(i)) return st.get(i);
        else return 0.0;
    }

    // nonzero entries in vector
    public int nnz() {
        return st.size();
    }

    public int dimension() {
        return d;
    }

    public String toCompressedString() {
        StringBuilder s = new StringBuilder();

        int i = st.keySet().size();
        String[] arr = new String[i];
        for (int cord : st.keySet()) {
            arr[--i] = String.format("(%d, %.2f)", cord, st.get(cord));
        }
        for (String j : arr) {
            s.append(j);
        }
        return s.toString();
    }

    @Override
    public String toString() {
        StringBuilder s = new StringBuilder();

        int i = st.keySet().size();
        double[] arr = new double[d];
        for (int cord : st.keySet()) {
            arr[cord] = st.get(cord);
        }
        for (double j : arr) {
            s.append(j).append(" ");
        }
        return s.toString();
    }
}
