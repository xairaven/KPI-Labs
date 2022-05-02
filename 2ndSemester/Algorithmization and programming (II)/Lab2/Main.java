package Lab2;
import edu.princeton.cs.algs4.In;
import edu.princeton.cs.algs4.StdRandom;
import Lab2.structures.*;

public class Main {
    public static void main(String[] args) {
        BST<String, SingleLinkedList<String>> countries = new BST<>();
        String[] countriesArr = {"Chezh", "England", "Finland", "France", "Italy", "Spain", "Switzerland", "Ukraine"};
        // pushing
        doingBST(countries, countriesArr);


        System.out.println("Countries in BST:");
        for (String s : countries.keys()) {
            System.out.print(s + " ");
        }
        System.out.println();

        // getting by key
        int randomNum = StdRandom.uniform(8);
        System.out.println("\nCities in random country: (Now it's " + countriesArr[randomNum] + ")");
        SingleLinkedList<String> randomCountry = countries.get(countriesArr[randomNum]);
        for (String s : randomCountry) {
            System.out.print(s + " ");
        }
        System.out.println();

        // searching
        System.out.println("\nDoes BST contains France? " + countries.contains("France"));
        System.out.println("Does BST contains Sweden? " + countries.contains("Sweden"));

        // deleting
        System.out.println("\nDeleting France from the list :(");
        countries.delete("France");
        System.out.println("Does BST contains France? " + countries.contains("France"));

        // checking height of the tree
        System.out.println("\nHeight of the tree: " + countries.height());

        // checking max length of the lists
        String countryWithMaxCities = "";
        int maxCities = 0;
        for (String s : countries.keys()) {
            int currentLength = countries.get(s).size();
            if (currentLength > maxCities) {
                countryWithMaxCities = s;
                maxCities = currentLength;
            }
        }
        System.out.println("\nCountry with max amount of cities is " + countryWithMaxCities + "!");
        System.out.println("This country has " + maxCities + " cities in a list.");
    }

    public static void doingBST(BST<String, SingleLinkedList<String>> countryBST, String[] countries) {
        StdRandom.shuffle(countries);
        for (String country : countries) {
            In in = new In("/Lab2/res/" + country + ".txt");
            SingleLinkedList<String> tempList = new SingleLinkedList<>();
            while (in.hasNextLine()) {
                tempList.addFirst(in.readLine());
            }
            countryBST.put(country, tempList);
        }
    }
}
