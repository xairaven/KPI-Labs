package Lab4.Task3;

import Lab4.Task3.Entities.*;
import Lab4.Task3.Enums.*;

public class Main {
    public static void main(String[] args) {
        var car = new Car(15, 1900, "Grey", 4, FuelType.Petrol);
        var bicycle = new Bicycle(5, 30, "Red", 2, BicycleType.Road);
        var bike = new Motorbike(10, 100, "Green", 2, MotoType.Sports);

        car.printInfo();

        System.out.println();

        bicycle.printInfo();

        System.out.println();

        bike.printInfo();
    }
}

