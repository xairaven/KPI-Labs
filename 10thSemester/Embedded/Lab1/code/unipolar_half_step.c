// Define CPU frequency as 16 MHz for the delay library
#define F_CPU 16000000UL

#include <avr/io.h>
#include <util/delay.h>

#define STEP_UP_AUTO (PINC & (1 << PINC0))
#define STEP_DOWN_AUTO (PINC & (1 << PINC1))
#define STEP_UP (PINC & (1 << PINC2))
#define STEP_DOWN (PINC & (1 << PINC3))

int check_up;
int check_down;
int angle;

void stepper() {
    if (angle == 0)
        PORTE = 0x09;
    if (angle == 45)
        PORTE = 0x08;
    if (angle == 90)
        PORTE = 0x0;
    if (angle == 135)
        PORTE = 0x04;
    if (angle == 180)
        PORTE = 0x06;
    if (angle == 225)
        PORTE = 0x02;
    if (angle == 270)
        PORTE = 0x03;
    if (angle == 315)
        PORTE = 0x01;
    if (angle == 360)
        PORTE = 0x09;
    if (angle == -45)
        PORTE = 0x01;
    if (angle == -90)
        PORTE = 0x03;
    if (angle == -135)
        PORTE = 0x02;
    if (angle == -180)
        PORTE = 0x06;
    if (angle == -225)
        PORTE = 0x04;
    if (angle == -270)
        PORTE = 0x0C;
    if (angle == -315)
        PORTE = 0x08;
    if (angle == -360)
        PORTE = 0x09;
}

void init_ports() {
    DDRC = 0x00;
    DDRE = 0xFF;
}

int main(void) {
    // Initialize variables here to avoid error
    // ELPM with no RAMPZ
    check_up = 1;
    check_down = 1;
    angle = 0;

    init_ports();
    while (1) {
        if (!STEP_UP_AUTO) {
            PORTE = 0x08;
            _delay_ms(1000);
            PORTE = 0x0C;
            _delay_ms(1000);
            PORTE = 0x04;
            _delay_ms(1000);
            PORTE = 0x06;
            _delay_ms(1000);
            PORTE = 0x02;
            _delay_ms(1000);
            PORTE = 0x03;
            _delay_ms(1000);
            PORTE = 0x01;
            _delay_ms(1000);
            PORTE = 0x09;
            //_delay_ms(1000);
        }
        if (!STEP_DOWN_AUTO) {
            PORTE = 0x01;
            _delay_ms(1000);
            PORTE = 0x03;
            _delay_ms(1000);
            PORTE = 0x02;
            _delay_ms(1000);
            PORTE = 0x06;
            _delay_ms(1000);
            PORTE = 0x04;
            _delay_ms(1000);
            PORTE = 0x0C;
            _delay_ms(1000);
            PORTE = 0x08;
            _delay_ms(1000);
            PORTE = 0x09;
            //_delay_ms(1000);
        }
        if (!STEP_UP && check_up == 1) {
            if (angle == 360) {
                angle = 0;
            }
            angle += 45;
            check_up = 0;
        }
        if (STEP_UP) {
            check_up = 1;
        }
        if (!STEP_DOWN && check_down == 1) {
            if (angle == -360) {
                angle = 0;
            }
            angle -= 45;
            check_down = 0;
        }
        if (STEP_DOWN) {
            check_down = 1;
        }
        stepper();
    }
}
