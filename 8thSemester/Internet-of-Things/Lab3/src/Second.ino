#define LED_PIN 2
#define BUTTONS_AMOUNT 10
#define CONFIRM_BUTTON_PIN 3

const int password = 17;
const int buttonPins[BUTTONS_AMOUNT] = {4, 5, 6, 7, 8, 9, 10, 11, 12, 13};
bool password_bin[BUTTONS_AMOUNT];
bool buttonState[BUTTONS_AMOUNT];

void setup() {
  pinMode(LED_PIN, OUTPUT);
  pinMode(CONFIRM_BUTTON_PIN, INPUT_PULLUP);

  for (int i = 0; i < BUTTONS_AMOUNT; i++) {
    pinMode(buttonPins[i], INPUT_PULLUP);
  }

  // Setting up password
  int tens = password / 10;
  int units = password % 10;
  if (tens > 0) {
    password_bin[tens] = true;
  }
  password_bin[units] = true;
}

void loop() {
  read_state();

  if (digitalRead(CONFIRM_BUTTON_PIN) == LOW) {
    bool isCorrect = check_state();
    digitalWrite(LED_PIN, isCorrect ? HIGH : LOW);
    if (isCorrect) {
      delay(500);
      digitalWrite(LED_PIN, LOW);
    }
    clear_state();
  }
}

void read_state() {
  for (int i = 0; i < BUTTONS_AMOUNT; i++) {
    if (!buttonState[i]) {
      buttonState[i] = (digitalRead(buttonPins[i]) == LOW); 
    }
  }
}

bool check_state() {
  for (int i = 0; i < BUTTONS_AMOUNT; i++) {
    if (buttonState[i] != password_bin[i]) {
      return false;
    }
  }
  return true;
}

void clear_state() {
  for (int i = 0; i < BUTTONS_AMOUNT; i++) {
    buttonState[i] = false;
  }
}
