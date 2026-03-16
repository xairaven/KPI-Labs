#define F_CPU 16000000L

#include <avr/io.h>
#include <avr/interrupt.h>
#include <avr/sleep.h>
#include <inttypes.h>
#include <util/delay.h>

int main(void) {
    // Init port B: pins 0, 1 for direction, pin 5 for PWM output
    DDRB = 0b00100011;
    PORTB = 0b00000000;

    // Init 16-bit timer 1 values for frequency generation
    OCR1A = 20;
    ICR1 = 40;
    int delay_step = 1;

    // Set Phase and frequency correct PWM mode (Mode 8)
    TCCR1B = (1 << WGM13);
    TCCR1A = 0;
    // Configure inverted PWM signal on OC1A (PB5)
    TCCR1A |= (1 << COM1A1) | (1 << COM1A0);
    // Start timer with prescaler N=8
    TCCR1B |= (1 << CS11);

    // Button state flags
    char flag_start_stop = 0, flag_reverse = 0, flag_speedup = 0, flag_speeddown = 0;

    while (1) {
        // Start-Stop button action handling (connected to PB2)
        if (!(PINB & (1 << PB2))) {
            flag_start_stop = 1;
            _delay_ms(10);
        }
        if ((flag_start_stop == 1) && (PINB & (1 << PB2))) {
            PORTB ^= (1 << PB0);
            flag_start_stop = 0;
        }

        // Reverse direction button action handling (connected to PB3)
        if (!(PINB & (1 << PB3))) {
            flag_reverse = 1;
            _delay_ms(10);
        }
        if ((flag_reverse == 1) && (PINB & (1 << PB3))) {
            PORTB ^= (1 << PB0) | (1 << PB1);
            flag_reverse = 0;
        }

        // Speed down button action handling (connected to PB4)
        if (!(PINB & (1 << PB4))) {
            flag_speeddown = 1;
            _delay_ms(10);
        }
        if ((flag_speeddown == 1) && (PINB & (1 << PB4))) {
            if (OCR1A != 40) {
                OCR1A += delay_step;
            }
            flag_speeddown = 0;
        }

        // Speed up button action handling (connected to PB6)
        if (!(PINB & (1 << PB6))) {
            flag_speedup = 1;
            _delay_ms(10);
        }
        if ((flag_speedup == 1) && (PINB & (1 << PB6))) {
            if (OCR1A != 0) {
                OCR1A -= delay_step;
            }
            flag_speedup = 0;
        }
    }
}
