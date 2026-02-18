package Lab4.Task3.Entities;

public abstract class Vehicle {
    protected double speed;
    protected double weight;
    protected String color;
    protected int amountOfWheels;

    public Vehicle(double speed, double weight, String color, int amountOfWheels) {
        setSpeed(speed);
        setWeight(weight);
        setColor(color);
        setAmountOfWheels(amountOfWheels);
    }

    public abstract double calculateMaxSpeed();
    public void printInfo() {
        System.out.println("-".repeat(30));
        System.out.println(this.toString());
        System.out.println("-".repeat(30));
    }

    @Override
    public String toString() {
        var sb = new StringBuilder();

        sb.append(String.format("Transport type: %s%n", this.getClass().getSimpleName()));
        sb.append(String.format("Speed: %.2f km/h%n", this.getSpeed()));
        sb.append(String.format("Weight: %.2f kg%n", this.getWeight()));
        sb.append(String.format("Amount of wheels: %d kg%n", this.getAmountOfWheels()));
        sb.append(String.format("Color: %s%n", this.getColor()));

        return sb.toString();
    }

    public double getSpeed() {
        return speed;
    }

    public void setSpeed(double speed) {
        if (speed < 0) {
            throw new IllegalArgumentException("Speed value must be non-negative");
        }
        this.speed = speed;
    }

    public double getWeight() {
        return weight;
    }

    public void setWeight(double weight) {
        if (weight < 0) {
            throw new IllegalArgumentException("Weight value must be non-negative");
        }
        this.weight = weight;
    }

    public String getColor() {
        return color;
    }

    public void setColor(String color) {
        if (color.trim().isEmpty()) {
            throw new IllegalArgumentException("Color value must be non-empty");
        }
        this.color = color;
    }

    public int getAmountOfWheels() {
        return amountOfWheels;
    }

    public void setAmountOfWheels(int amountOfWheels) {
        if (amountOfWheels < 0) {
            throw new IllegalArgumentException("Wheels numbers value must be non-negative");
        }
        this.amountOfWheels = amountOfWheels;
    }
}
