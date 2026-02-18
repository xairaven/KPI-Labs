package Lab4.Task3.Entities;

import Lab4.Task3.Enums.FuelType;

public class Car extends Vehicle {
    protected FuelType fuelType;

    public Car(double speed, double weight, String color, int numberOfWheels, FuelType fuelType) {
        super(speed, weight, color, numberOfWheels);

        setFuelType(fuelType);
    }

    public FuelType getFuelType() {
        return fuelType;
    }

    public void setFuelType(FuelType fuelType) {
        this.fuelType = fuelType;
    }

    @Override
    public double calculateMaxSpeed() {
        return speed + switch (fuelType) {
            case Electro -> 5;
            case Ethanol -> 10;
            case BioDiesel -> 15;
            case Diesel -> 20;
            case Petrol -> 25;
        };
    }

    @Override
    public String toString() {
        var sb = new StringBuilder();

        sb.append(super.toString());
        sb.append(String.format("Fuel type: %s", this.getFuelType()));

        return sb.toString();
    }
}
