#include <Wire.h>
#include <Adafruit_GFX.h>
#include <Adafruit_SSD1306.h>

#define SCREEN_WIDTH 128 
#define SCREEN_HEIGHT 64  
#define ADC_PIN A0 

Adafruit_SSD1306 display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, -1);

void setup() {
    if (!display.begin(SSD1306_SWITCHCAPVCC, 0x3C)) { 
        for (;;); 
    }

    display.clearDisplay();
    display.setTextSize(1);
    display.setTextColor(WHITE);
}

void loop() {
    int adcValue = analogRead(ADC_PIN); 
    float voltage = adcValue * (5.0 / 1023.0); 
    float percentage = (adcValue / 1023.0) * 100.0; 

    display.clearDisplay();
    
    display.setCursor(0, 10);
    display.setTextSize(1);
    display.print("ADC: ");
    display.println(adcValue);

    display.setCursor(0, 30);
    display.setTextSize(1);
    display.print("Volt: ");
    display.print(voltage, 2);
    display.println("V");

    display.setCursor(0, 50);
    display.setTextSize(1);
    display.print("Perc: ");
    display.print(percentage, 1);
    display.println("%");

    display.display();
    
    delay(500);  
}