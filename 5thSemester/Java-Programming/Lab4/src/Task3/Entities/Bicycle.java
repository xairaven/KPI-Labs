package Lab4.Task3.Entities;

import Lab4.Task3.Enums.BicycleType;

public class Bicycle extends Vehicle {
    protected BicycleType type;

    public Bicycle(double speed, double weight, String color, int numberOfWheels, BicycleType type) {
        super(speed, weight, color, numberOfWheels);

        setType(type);
    }

    @Override
    public double calculateMaxSpeed() {
        return speed + switch (type) {
            case Road, Mountain, Gravel, Track -> 15;
            case Kids, Fat, Tricycle, Tandem -> 0;
            case EBike, Utility, Fitness -> 2;
            case BMX, Triathlon -> 25;
        };
    }

    @Override
    public String toString() {
        var sb = new StringBuilder();

        sb.append(super.toString());
        sb.append(String.format("Type: %s", this.getType()));

        return sb.toString();
    }

    public BicycleType getType() {
        return type;
    }

    public void setType(BicycleType type) {
        this.type = type;
    }
}
