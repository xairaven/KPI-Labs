package Lab3;

import Lab3.structs.DoublyLinkedList;
import Lab3.structs.List;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.Scanner;

public class Hairdressers {
    public String label;
    public String address;
    public int numberOfWorkers;
    public String workTime;

    public Hairdressers(String label, String address, int numberOfWorkers, String workTime) {
        this.label = label;
        this.address = address;
        this.numberOfWorkers = numberOfWorkers;
        this.workTime = workTime;
    }

    public static List readFromFile(String file) {
        var list = new List();
        try {
            File myObj = new File(file);
            Scanner myReader = new Scanner(myObj);
            while (myReader.hasNextLine()) {
                String[] data = myReader.nextLine().split(", ");

                String label = data[0].split(": ")[1];
                String address = data[1].split(": ")[1];
                int workers = Integer.parseInt(data[2].split(": ")[1]);
                String workTime = data[3].split(": ")[1];

                list.push(new Hairdressers(label, address, workers, workTime));
            }
            myReader.close();
        } catch (FileNotFoundException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }

        return list;
    }

    @Override
    public String toString() {
        return "Label: " + label + ", " +
                "Address: " + address + ", " +
                "Number of workers: " + numberOfWorkers + ", " +
                "Work time: " + workTime;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null) return false;

        if (getClass() != o.getClass()) return false;
        Hairdressers item = (Hairdressers) o;
        // field comparison
        return label.equals(item.label)
                && address.equals(item.address)
                && numberOfWorkers == item.numberOfWorkers
                && workTime.equals(item.workTime);
    }
}
