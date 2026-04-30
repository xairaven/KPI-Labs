#define F_CPU 8000000UL
#include <avr/io.h>
#include <util/delay.h>
#include <stdio.h> // For sprintf function

// ================= 1-WIRE SETUP =================
#define DQ_PIN PA0
#define DQ_PORT PORTA
#define DQ_DDR DDRA
#define DQ_PIN_REG PINA

unsigned char W1_Init(void) {
    unsigned char state;
    DQ_DDR |= (1 << DQ_PIN);    // Set pin as output
    DQ_PORT &= ~(1 << DQ_PIN);  // Pull bus low
    _delay_us(480);             // Reset pulse
    DQ_DDR &= ~(1 << DQ_PIN);   // Release bus (set as input)
    DQ_PORT &= ~(1 << DQ_PIN);  // Disable internal pull-up
    _delay_us(70);              // Wait for response
    state = (DQ_PIN_REG & (1 << DQ_PIN)) ? 1 : 0; // Read line state
    _delay_us(410);             // Wait for time slot to complete
    return state;
}

void W1_WriteByte(unsigned char data) {
    unsigned char i;
    for (i = 0; i < 8; i++) {
        DQ_DDR |= (1 << DQ_PIN);
        DQ_PORT &= ~(1 << DQ_PIN);
        _delay_us(2);
        if (data & (1 << i)) {
            DQ_DDR &= ~(1 << DQ_PIN); // Release for "1"
        }
        _delay_us(60);
        DQ_DDR &= ~(1 << DQ_PIN); // Release after bit transmission
        _delay_us(2);
    }
}

unsigned char W1_ReadByte(void) {
    unsigned char i, data = 0;
    for (i = 0; i < 8; i++) {
        DQ_DDR |= (1 << DQ_PIN);
        DQ_PORT &= ~(1 << DQ_PIN);
        _delay_us(2);
        DQ_DDR &= ~(1 << DQ_PIN); // Release bus
        _delay_us(10);            // Wait for data from sensor
        if (DQ_PIN_REG & (1 << DQ_PIN)) {
            data |= (1 << i);
        }
        _delay_us(50);
    }
    return data;
}

// ================= LCD SETUP (4-bit) =================
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
    if (mode) LCD_PORT |= (1 << RS); // Data mode
    else LCD_PORT &= ~(1 << RS);     // Command mode

    // Send high 4 bits
    LCD_PORT = (LCD_PORT & 0x0F) | (value & 0xF0);
    LCD_Pulse();

    // Send low 4 bits
    LCD_PORT = (LCD_PORT & 0x0F) | ((value << 4) & 0xF0);
    LCD_Pulse();
    _delay_ms(2);
}

void LCD_Init(void) {
    LCD_DDR = 0xFF; // Port C as output
    _delay_ms(20);
    
    // HD44780 initialization procedure
    LCD_PORT = (LCD_PORT & 0x0F) | 0x30;
    LCD_Pulse(); _delay_ms(5);
    LCD_Pulse(); _delay_us(150);
    LCD_Pulse(); _delay_us(150);

    LCD_PORT = (LCD_PORT & 0x0F) | 0x20; // Switch to 4-bit mode
    LCD_Pulse(); _delay_ms(2);

    LCD_Send(0x28, 0); // 4-bit, 2 lines, 5x8 font
    LCD_Send(0x0C, 0); // Display ON, Cursor OFF
    LCD_Send(0x01, 0); // Clear display
    _delay_ms(2);
}

void LCD_String(char* str) {
    while (*str) {
        LCD_Send(*str++, 1);
    }
}

// ================= MAIN PROGRAM =================
int main(void) {
    unsigned char t_low, t_high;
    int temp16;
    unsigned char temp_int;
    unsigned char frac_raw;
    unsigned int frac_decimal;
    char buffer[16];

    LCD_Init();
    LCD_String("1-Wire System");
    LCD_Send(0xC0, 0); // Move to second line
    LCD_String("Loading...");
    _delay_ms(1000);

    while (1) {
        LCD_Send(0x01, 0); // Clear display

        // If sensor is present (responded with 0)
        if (W1_Init() == 0) { 
            // 1. Start conversion command
            W1_WriteByte(0xCC); // Skip ROM (only one device on bus)
            W1_WriteByte(0x44); // Start conversion (Convert T)
            
            _delay_ms(750);     // DS18B20 requires up to 750ms for 12-bit conversion

            // 2. Read data command
            W1_Init();
            W1_WriteByte(0xCC); // Skip ROM
            W1_WriteByte(0xBE); // Read Scratchpad

            t_low = W1_ReadByte();  // Low byte
            t_high = W1_ReadByte(); // High byte

            temp16 = (t_high << 8) | t_low; // Combine into 16 bits

            LCD_String("Temp: ");

            // Sign check
            if ((temp16 & 0x8000) == 0) {
                LCD_Send('+', 1);
            } else {
                LCD_Send('-', 1);
                temp16 = ~temp16 + 1; // Invert value for negative temperature
            }

            // Shift right by 4 to get integer part
            temp_int = (unsigned char)(temp16 >> 4);
            
            // Fractional part
            frac_raw = temp16 & 0x0F;
            frac_decimal = (frac_raw * 10) / 16; 

            // Display the result
            sprintf(buffer, "%d.%d C", temp_int, frac_decimal);
            LCD_String(buffer);
        } else {
            LCD_String("Sensor Error!");
        }

        _delay_ms(500); // Delay before next polling
    }
    return 0;
}