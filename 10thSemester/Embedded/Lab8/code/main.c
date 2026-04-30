#define F_CPU 8000000UL
#include <avr/io.h>
#include <util/delay.h>
#include <stdio.h>

#define LCD_PORT PORTC
#define LCD_DDR DDRC
#define RS PC0
#define EN PC1

void LCD_Pulse(void) {
    LCD_PORT |= (1 << EN);
    _delay_us(1);
    LCD_PORT &= ~(1 << EN);
    _delay_us(50);
}

void LCD_Send(unsigned char value, unsigned char mode) {
    if (mode) LCD_PORT |= (1 << RS);
    else LCD_PORT &= ~(1 << RS);

    LCD_PORT = (LCD_PORT & 0x0F) | (value & 0xF0);
    LCD_Pulse();

    LCD_PORT = (LCD_PORT & 0x0F) | ((value << 4) & 0xF0);
    LCD_Pulse();
    _delay_ms(2);
}

void LCD_Init(void) {
    LCD_DDR = 0xFF;
    _delay_ms(20);
    
    LCD_PORT = (LCD_PORT & 0x0F) | 0x30;
    LCD_Pulse(); _delay_ms(5);
    LCD_Pulse(); _delay_us(150);
    LCD_Pulse(); _delay_us(150);

    LCD_PORT = (LCD_PORT & 0x0F) | 0x20; 
    LCD_Pulse(); _delay_ms(2);

    LCD_Send(0x28, 0);
    LCD_Send(0x0C, 0);
    LCD_Send(0x01, 0);
    _delay_ms(2);
}

void LCD_String(char* str) {
    while (*str) {
        LCD_Send(*str++, 1);
    }
}

void LCD_SetCursor(unsigned char col, unsigned char row) {
    unsigned char address = (row == 0) ? (0x80 + col) : (0xC0 + col);
    LCD_Send(address, 0);
}

void ADC_Init(void) {
    // Set AVCC as reference voltage and select ADC0 channel
    ADMUX = (1 << REFS0);
    
    // Enable ADC and set prescaler to 128 for optimal clock
    ADCSRA = (1 << ADEN) | (1 << ADPS2) | (1 << ADPS1) | (1 << ADPS0);
}

uint16_t ADC_Read(void) {
    // Start analog to digital conversion
    ADCSRA |= (1 << ADSC);
    
    // Wait until conversion is fully complete
    while (ADCSRA & (1 << ADSC));
    
    // Return the 10-bit result directly from ADC register
    return ADC; 
}

int main(void) {
    uint16_t adc_value;
    uint8_t volts;
    uint8_t millivolts;
    char buffer[16];

    LCD_Init();
    ADC_Init();
    
    LCD_SetCursor(0, 0);
    LCD_String("Voltmeter:");

    while (1) {
        adc_value = ADC_Read();

        // Convert 10-bit raw value to voltage avoiding float operations
        uint32_t temp = ((uint32_t)adc_value * 500) / 1023; 
        
        volts = temp / 100;
        millivolts = temp % 100;

        LCD_SetCursor(0, 1);
        sprintf(buffer, "U = %d.%02d V   ", volts, millivolts);
        LCD_String(buffer);

        _delay_ms(200);
    }
    return 0;
}