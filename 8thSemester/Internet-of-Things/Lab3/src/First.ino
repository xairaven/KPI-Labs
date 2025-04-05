int VAR = 17;

void setup() {
    // No need to do anything here for now
}

int fibonacci(int n) {
    if (n <= 1) {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

void output(int n) {
    for (int i = 0; n > 0; i++) {
        pinMode(i, OUTPUT);
        if (n % 2) {
            digitalWrite(i, 1);
        }
        n /= 2;
    }

    delay(500);

    for (int i = 0; i < 8; i++) {
        digitalWrite(i, 0);
    }

    delay(500);
}

void loop() {
    static int i = 1;  // Keep track of Fibonacci index
    int fib = fibonacci(i);
    
    if (fib > VAR) {
        i = 1;  // Restart when exceeding VAR
    } else {
        output(fib);
        i++;
    }
}
