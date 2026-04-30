#define F_CPU 8000000UL
#include <avr/io.h>
#include <util/delay.h>
#include <stdio.h>

// ================= LCD DRIVER (HD44780) =================
#define LCD_PORT PORTC
#define LCD_DDR DDRC
#define RS PC0
#define EN PC1

// Strobe pulse generation function (write data to display)
void LCD_Pulse(void) {
    LCD_PORT |= (1 << EN);   // Set EN to 1
    _delay_us(1);            // Short delay
    LCD_PORT &= ~(1 << EN);  // Reset EN to 0 (data is latched on falling edge)
    _delay_us(50);
}

// Function to send a byte (command or data) in 4-bit mode
void LCD_Send(unsigned char value, unsigned char mode) {
    if (mode) LCD_PORT |= (1 << RS); // Data mode (RS = 1)
    else LCD_PORT &= ~(1 << RS);     // Command mode (RS = 0)

    // Send high nibble (4 bits)
    LCD_PORT = (LCD_PORT & 0x0F) | (value & 0xF0);
    LCD_Pulse();

    // Send low nibble (4 bits)
    LCD_PORT = (LCD_PORT & 0x0F) | ((value << 4) & 0xF0);
    LCD_Pulse();
    _delay_ms(2); // Command execution delay
}

// Display initialization function
void LCD_Init(void) {
    LCD_DDR = 0xFF; // Configure Port C as output
    _delay_ms(20);  // Wait for display power stabilization
    
    // Mandatory reset sequence for HD44780
    LCD_PORT = (LCD_PORT & 0x0F) | 0x30;
    LCD_Pulse(); _delay_ms(5);
    LCD_Pulse(); _delay_us(150);
    LCD_Pulse(); _delay_us(150);

    // Switch display to 4-bit mode
    LCD_PORT = (LCD_PORT & 0x0F) | 0x20; 
    LCD_Pulse(); _delay_ms(2);

    // Parameter configuration
    LCD_Send(0x28, 0); // 0x28: 4-bit interface, 2 lines, 5x8 font
    LCD_Send(0x0C, 0); // 0x0C: Display ON, cursor OFF (not blinking)
    LCD_Send(0x01, 0); // 0x01: Clear screen completely
    _delay_ms(2);
}

// Function to print a string of text
void LCD_String(char* str) {
    while (*str) {
        LCD_Send(*str++, 1); // Send each character one by one
    }
}

// Function to set cursor (col - column 0..15, row - line 0..1)
void LCD_SetCursor(unsigned char col, unsigned char row) {
    // 0x80 - base address of the first line, 0xC0 - second line
    unsigned char address = (row == 0) ? (0x80 + col) : (0xC0 + col);
    LCD_Send(address, 0);
}

// ================= 1-WIRE DRIVER (DS18B20) =================
#define DQ_PIN PA0
#define DQ_PORT PORTA
#define DQ_DDR DDRA
#define DQ_PIN_REG PINA

unsigned char W1_Init(void) {
    unsigned char state;
    DQ_DDR |= (1 << DQ_PIN);
    DQ_PORT &= ~(1 << DQ_PIN);
    _delay_us(480);
    DQ_DDR &= ~(1 << DQ_PIN);
    DQ_PORT &= ~(1 << DQ_PIN);
    _delay_us(70);
    state = (DQ_PIN_REG & (1 << DQ_PIN)) ? 1 : 0;
    _delay_us(410);
    return state;
}

void W1_WriteByte(unsigned char data) {
    unsigned char i;
    for (i = 0; i < 8; i++) {
        DQ_DDR |= (1 << DQ_PIN);
        DQ_PORT &= ~(1 << DQ_PIN);
        _delay_us(2);
        if (data & (1 << i)) DQ_DDR &= ~(1 << DQ_PIN);
        _delay_us(60);
        DQ_DDR &= ~(1 << DQ_PIN);
        _delay_us(2);
    }
}

unsigned char W1_ReadByte(void) {
    unsigned char i, data = 0;
    for (i = 0; i < 8; i++) {
        DQ_DDR |= (1 << DQ_PIN);
        DQ_PORT &= ~(1 << DQ_PIN);
        _delay_us(2);
        DQ_DDR &= ~(1 << DQ_PIN);
        _delay_us(10);
        if (DQ_PIN_REG & (1 << DQ_PIN)) data |= (1 << i);
        _delay_us(50);
    }
    return data;
}

// ================= MAIN PROGRAM =================
int main(void) {
    unsigned char t_low, t_high;
    int temp16;
    unsigned char temp_int;
    unsigned int frac_decimal;
    char buffer[16];

    // Initialize display
    LCD_Init();
    
    // Beautiful startup screen (LCD demonstration)
    LCD_SetCursor(1, 0); // Offset 1 character, 1st line
    LCD_String("Lab 7: LCD");
    LCD_SetCursor(0, 1); // Start of 2nd line
    LCD_String("Initializing...");
    _delay_ms(1500);

    while (1) {
        if (W1_Init() == 0) { 
            // Start temperature conversion
            W1_WriteByte(0xCC); 
            W1_WriteByte(0x44); 
            _delay_ms(750);     

            // Read result
            W1_Init();
            W1_WriteByte(0xCC); 
            W1_WriteByte(0xBE); 

            t_low = W1_ReadByte();  
            t_high = W1_ReadByte(); 
            temp16 = (t_high << 8) | t_low; 

            // Clear screen before new output
            LCD_Send(0x01, 0); 
            
            // Print static text on the first line
            LCD_SetCursor(0, 0);
            LCD_String("Temperature:");

            // Move to the second line to print the value
            LCD_SetCursor(0, 1);

            if ((temp16 & 0x8000) == 0) {
                LCD_Send('+', 1); // '+' character
            } else {
                LCD_Send('-', 1); // '-' character
                temp16 = ~temp16 + 1; 
            }

            temp_int = (unsigned char)(temp16 >> 4);
            frac_decimal = ((temp16 & 0x0F) * 10) / 16; 

            // Formatting and output
            sprintf(buffer, "%d.%d C", temp_int, frac_decimal);
            LCD_String(buffer);
            
        } else {
            LCD_Send(0x01, 0);
            LCD_String("Sensor Error!");
        }

        _delay_ms(500); 
    }
    return 0;
}