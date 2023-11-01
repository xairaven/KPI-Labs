package Lab3.Task1;

public class BruteForceCalculator {
    private int cardinality;
    private int passwordLength;
    private int speed;

    public BruteForceCalculator(int cardinality, int passwordLength, int speed) {
        setCardinality(cardinality);
        setPasswordLength(passwordLength);
        setSpeed(speed);
    }

    public long seconds() {
        long systemCardinality = (long) Math.pow(cardinality, passwordLength);

        return systemCardinality / speed;
    }

    public int getCardinality() {
        return cardinality;
    }

    public void setCardinality(int cardinality) {
        BruteForceValidator.cardinality(cardinality);

        this.cardinality = cardinality;
    }

    public int getPasswordLength() {
        return passwordLength;
    }

    public void setPasswordLength(int passwordLength) {
        BruteForceValidator.passwordLength(passwordLength);

        this.passwordLength = passwordLength;
    }

    public int getSpeed() {
        return speed;
    }

    public void setSpeed(int speed) {
        BruteForceValidator.speed(speed);

        this.speed = speed;
    }
}