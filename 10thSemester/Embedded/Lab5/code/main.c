#include <avr/io.h>
#include <avr/interrupt.h>

// Sine wave values table (256 values centered at 128)
const unsigned char SINE_TABLE[256] = {
    64,65,67,68,70,72,73,75,76,78,79,81,82,84,85,87,
    88,90,91,92,94,95,97,98,99,100,102,103,104,105,107,108,
    109,110,111,112,113,114,115,116,117,118,118,119,120,121,121,122,
    123,123,124,124,125,125,126,126,126,127,127,127,127,127,127,127,
    128,127,127,127,127,127,127,127,126,126,126,125,125,124,124,123,
    123,122,121,121,120,119,118,118,117,116,115,114,113,112,111,110,
    109,108,107,105,104,103,102,100,99,98,97,95,94,92,91,90,
    88,87,85,84,82,81,79,78,76,75,73,72,70,68,67,65,
    64,62,61,59,58,56,54,53,51,50,48,47,45,44,42,41,
    39,38,36,35,34,32,31,30,28,27,26,25,23,22,21,20,
    19,18,17,15,14,13,13,12,11,10,9,8,8,7,6,5,
    5,4,4,3,3,2,2,2,1,1,1,0,0,0,0,0,
    0,0,0,0,0,0,1,1,1,2,2,2,3,3,4,4,
    5,5,6,7,8,8,9,10,11,12,13,13,14,15,17,18,
    19,20,21,22,23,25,26,27,28,30,31,32,34,35,36,38,
    39,41,42,44,45,47,48,50,51,53,54,56,58,59,61,62
};

// Seven segment digit codes (Custom wiring: PC7=a, PC6=b, PC5=c, PC4=d, PC3=e, PC2=f, PC1=g)
const unsigned char SEGMENT_CODE[10] = {
    0xFC, // 0
    0x60, // 1
    0xDA, // 2
    0xF2, // 3
    0x66, // 4
    0xB6, // 5
    0xBE, // 6
    0xE0, // 7
    0xFE, // 8
    0xF6  // 9
};

// Masks for selecting digits on PORTD (PD7 is leftmost digit, PD4 is rightmost)
const unsigned char DIGIT_SELECT[4] = {0x70, 0xB0, 0xD0, 0xE0};

volatile unsigned int current_frequency = 500;
volatile unsigned int phase_accumulator = 0;
volatile unsigned int tuning_word = 0;

volatile unsigned char display_buffer[4] = {0, 0, 0, 0};
volatile unsigned char active_digit = 0;

// Update tuning word and display buffer based on new frequency
void set_frequency(unsigned int freq) 
{
    // Limit frequency boundaries
    if (freq > 9999) freq = 9999;
    if (freq < 10) freq = 10;
    
    current_frequency = freq;
    
    // Calculate DDS tuning word: (freq * 2^16) / sample_rate
    // Sample rate = 8000000 / 256 = 31250 Hz. Constant is roughly 2.097
    tuning_word = (current_frequency * 2147UL) >> 10;
    
    // Update display digits
    display_buffer[0] = SEGMENT_CODE[current_frequency / 1000];
    display_buffer[1] = SEGMENT_CODE[(current_frequency / 100) % 10];
    display_buffer[2] = SEGMENT_CODE[(current_frequency / 10) % 10];
    display_buffer[3] = SEGMENT_CODE[current_frequency % 10];

	// Blank the leading zero if frequency is below 1000
    if (current_frequency < 1000) {
        display_buffer[0] = 0x00; 
    }
}

// Timer0 Overflow Interrupt: Sine wave generation (Runs at ~31.25 kHz)
ISR(TIMER0_OVF_vect) 
{
    phase_accumulator += tuning_word;
    PORTA = SINE_TABLE[phase_accumulator >> 8];
}

// Timer1 Compare Match Interrupt: Display multiplexing (Runs at 1 kHz)
ISR(TIMER1_COMPA_vect) 
{
    // Turn off all digits to prevent ghosting
    PORTD |= 0xF0; 
    
    // Output segment data
    PORTC = display_buffer[active_digit];
    
    // Turn on the active digit while preserving other bits
    PORTD = (PORTD & 0x0F) | DIGIT_SELECT[active_digit];
    
    // Move to the next digit
    active_digit++;
    if (active_digit > 3) active_digit = 0;
}

// External Interrupt 0: Increase frequency
ISR(INT0_vect) 
{
    set_frequency(current_frequency + 50);
}

// External Interrupt 1: Decrease frequency
ISR(INT1_vect) 
{
    set_frequency(current_frequency - 50);
}

int main() 
{
    // Configure Ports
    DDRA = 0xFF;  // PORTA as output for DAC
    DDRC = 0xFF;  // PORTC as output for 7-segment data
    DDRD = 0xF0;  // PD4-PD7 output for digit selection, PD2-PD3 input for buttons
    
    // Enable pull-up resistors on PD2, PD3 and disable all digits initially
    PORTD = 0xFC; 
    
    // Configure Timer0 for DDS generation (Normal mode, No prescaler)
    TCCR0 = (1 << CS00);
    TIMSK |= (1 << TOIE0);
    
    // Configure Timer1 for display multiplexing (CTC mode, Prescaler 64)
    TCCR1B = (1 << WGM12) | (1 << CS11) | (1 << CS10);
    OCR1A = 124; // 8MHz / 64 / 125 = 1000 Hz refresh rate
    TIMSK |= (1 << OCIE1A);
    
    // Configure External Interrupts for buttons (Trigger on falling edge)
    MCUCR = (1 << ISC11) | (1 << ISC01);
    GICR = (1 << INT1) | (1 << INT0);
    
    // Initialize starting frequency
    set_frequency(500);
    
    // Enable global interrupts
    sei();
    
    // Infinite loop, everything is interrupt-driven
    while(1) {
    }
    
    return 0;
}