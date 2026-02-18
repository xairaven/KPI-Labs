package Lab4.Task3.Entities;

import Lab4.Task3.Enums.MotoType;

public class Motorbike extends Vehicle {
    protected MotoType type;

    public Motorbike(double speed, double weight, String color, int numberOfWheels, MotoType type) {
        super(speed, weight, color, numberOfWheels);

        setType(type);
    }

    @Override
    public double calculateMaxSpeed() {
        return speed + switch (type) {
            case Standard, DualPurpose -> 0;
            case Cruiser, Touring, OffRoad -> 5;
            case Sports -> 15;
        };
    }

    @Override
    public String toString() {
        var sb = new StringBuilder();

        sb.append(super.toString());
        sb.append(String.format("Type: %s", this.getType()));

        return sb.toString();
    }

    public MotoType getType() {
        return type;
    }

    public void setType(MotoType type) {
        this.type = type;
    }
}
