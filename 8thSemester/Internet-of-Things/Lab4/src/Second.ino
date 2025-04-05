#include <Wire.h>
#include <RTClib.h>       
#include <Adafruit_SSD1306.h>
#include <DHT.h>          

#define SCREEN_WIDTH 128
#define SCREEN_HEIGHT 64
Adafruit_SSD1306 display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, -1);

RTC_DS1307 rtc;

#define DHTPIN 2       
#define DHTTYPE DHT22  
DHT dht(DHTPIN, DHTTYPE);

void setup() {
  Serial.begin(9600);

  if (!display.begin(SSD1306_SWITCHCAPVCC, 0x3C)) { 
    Serial.println("OLED display not found");
    while (1);
  }

  display.clearDisplay();
  display.setTextSize(1);
  display.setTextColor(SSD1306_WHITE);
    
  if (!rtc.begin()) {
    Serial.println("RTC module not found!");
    while (1);
  }

  if (!rtc.isrunning()) {
    Serial.println("RTC is not set! Setting to compile time...");
    rtc.adjust(DateTime(F(__DATE__), F(__TIME__)));
  }

  dht.begin();
}

void loop() {
  DateTime now = rtc.now(); 
  float temperature = dht.readTemperature(); 

  if (isnan(temperature)) {
    Serial.println("DHT reading error");
    return;
  }

  String state;
  if (temperature < 16) {
    state = "Cold";
  } else if (temperature > 28) {
    state = "Hot";
  } else {
    state = "Comfortable";
  }

  Serial.print("Temperature: ");
  Serial.print(temperature);
  Serial.print(" C, Status: ");
  Serial.println(state);

  display.clearDisplay();
  display.setCursor(0, 0);
  display.print("Time: ");
  display.print(now.hour());
  display.print(":");
    
  if (now.minute() < 10) display.print("0"); 
  display.print(now.minute());
    
  display.setCursor(0, 20);
  display.print("Temperature: ");
  display.print(temperature);
  display.print(" C");

  display.setCursor(0, 40);
  display.print("Status: ");
  display.print(state);

  display.display(); 

  delay(60000); 
}
