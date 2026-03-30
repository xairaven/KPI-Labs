#include <avr/io.h>
#include <avr/interrupt.h>

#define TRUE 1
#define FALSE 0

/* Clock operating modes */
#define MODE_WORK 0
#define MODE_EDIT_SECS 1
#define MODE_EDIT_MINS 2
#define MODE_EDIT_HOURS 3

/* Limits for hours, minutes, and seconds in BCD code */
#define LIMIT_HOURS 0x24
#define LIMIT_MINUTES 0x60
#define LIMIT_SECONDS 0x60

/* Array indices for current time components */
#define I_SECONDS 0
#define I_MINUTES 1
#define I_HOURS 2

/* Number of digits on the display */
#define N_DIGITS 6

/* CPU frequency */
#define F_CPU (4000000UL)

/* Right shifts to move the high nibble to the low nibble */
#define HI_NIBBLE_SHIFT 0x04

/* Mask to extract the low nibble */
#define LO_NIBBLE_MASK 0x0F

/* BCD correction constant when reaching 0xXA */
#define BCD_CORRECTION 0x06

/* Mask for the +1 button pin (PE6) */
#define PLUS_BUTTON_MASK 0x40

/* Delay for holding the +1 button before auto-increment */
#define AUTOINC_DELAY 2

/* Port E initialization mask */
#define PE_INIT_MASK 0x3F

/* Mask to turn off the display (preserves pull-up resistors on PE6, PE7) */
#define INDICATOR_DESELECT_MASK 0xC0

/* Display refresh rate in Hz */
#define REFRESH_RATE 100

/* T/C1 prescaler */
#define PRESCALER 64

/* Display segments */
#define A 1   // 0x01
#define B 2   // 0x02
#define C 4   // 0x04
#define D 8   // 0x08
#define E 16  // 0x10
#define F 32  // 0x20
#define G 64  // 0x40
#define DP 128 // 0x80

/* Seven-segment encoding table */
unsigned char LED_CODE[10];

/* T/C1 limit and Blink period as defines */
#define TIMER_MAX (F_CPU / PRESCALER / REFRESH_RATE / N_DIGITS)
#define BLINK_MAX 150

/* Global variables declared without initialization */
unsigned char updateCounter;
unsigned char blinkMask;
unsigned char newBlinkMask;
unsigned char time[3];
unsigned char lastRefreshedDigit;
unsigned char mode;
unsigned char holdCounter;

/* * Increments a specific time component in the array.
 * Returns TRUE if overflow occurs.
 */
unsigned char incTimePosition(unsigned char index, unsigned char limit)
{
    if (index < N_DIGITS >> 1) {
        time[index]++;

        /* BCD counting: add 0x06 when reaching 0x0A */
        if ((time[index] & LO_NIBBLE_MASK) == 0x0A) {
            time[index] += BCD_CORRECTION;
        }

        /* Reset to 0 when reaching the limit */
        if (time[index] == limit) {
            time[index] = 0;
            return TRUE;
        }
    }
    return FALSE;
}

/* Refreshes the display digits */
void refreshDigit()
{
    unsigned char index = lastRefreshedDigit >> 1;

    /* Determine if high or low nibble is needed, shifting accordingly */
    unsigned char shift = (lastRefreshedDigit & 0x01) ? HI_NIBBLE_SHIFT : 0x00;

    /* Determine if the decimal point is needed */
    unsigned char point = (shift == 0 && index > 0) ? DP : 0x00;

    /* Turn off the display */
    PORTE &= INDICATOR_DESELECT_MASK;

    /* Output the next digit to Port D */
    PORTD = LED_CODE[(time[index] >> shift) & LO_NIBBLE_MASK] | point;

    /* Select the display position applying the blink mask */
    PORTE |= (1 << lastRefreshedDigit) & blinkMask;

    /* Reset after updating the last digit */
    if (++lastRefreshedDigit == N_DIGITS) {
        /* Update the blink mask after a full display cycle to avoid desync */
        blinkMask = newBlinkMask;
        lastRefreshedDigit = 0;
    }
}

/* Increment hours */
void incHours()
{
    incTimePosition(I_HOURS, LIMIT_HOURS);
}

/* Increment minutes */
void incMinutes()
{
    /* Minutes overflowed -> increment hours */
    if (incTimePosition(I_MINUTES, LIMIT_MINUTES)) {
        incHours();
    }
}

/* Increment seconds */
void incSeconds()
{
    /* Seconds overflowed -> increment minutes */
    if (incTimePosition(I_SECONDS, LIMIT_SECONDS)) {
        incMinutes();
    }
}

/* Increment the currently selected time component in setup mode */
void setupIncTime()
{
    if (mode == MODE_EDIT_HOURS) {
        incTimePosition(I_HOURS, LIMIT_HOURS);
    } else if (mode == MODE_EDIT_MINS) {
        incTimePosition(I_MINUTES, LIMIT_MINUTES);
    } else {
        incTimePosition(I_SECONDS, LIMIT_SECONDS);
    }
}

/* Asynchronous timer T/C0 interrupt handler (1 Hz) */
ISR(TIMER0_OVF_vect) 
{
    /* SIMULIDE FIX: The simulator ignores async mode and runs T/C0 from the 4MHz main clock. 
     * 4,000,000 / 128 (prescaler) / 256 (overflow) = ~122 interrupts per second.
     * We add a software divider to count 122 interrupts before ticking 1 real second.
     */
    static unsigned char sim_divider = 0;
    
    if (++sim_divider >= 122) {
        sim_divider = 0;
        if (!mode) {
            /* Increment time if not in setup mode */
            incSeconds();
        }
    }
}

/* T/C1 interrupt handler (Display multiplexing) */
ISR(TIMER1_COMPA_vect)
{
    /* Refresh a digit on the display */
    refreshDigit();

    if (mode == 0) {
        /* Do nothing else if in normal work mode */
        return;
    }

    /* Increment refresh counter */
    updateCounter++;

    /* Toggle blink state if BLINK_MAX is reached */
    if (updateCounter == BLINK_MAX) {

        /* Is the +1 button pressed? */
        if (PINE & PLUS_BUTTON_MASK) {
            /* Not pressed */
            /* Invert 2 bits in the blink mask depending on the edit mode */
            newBlinkMask ^= ((3 << ((mode - 1) << 1)));

            /* Reset hold counter */
            holdCounter = 0;
        } else {
            /* Pressed */
            /* Start auto-increment if the button is held for AUTOINC_DELAY cycles */
            if (holdCounter == AUTOINC_DELAY) {
                setupIncTime();
            } else {
                holdCounter++;
            }
            /* Disable blinking during auto-increment */
            newBlinkMask = 0xFF;
        }

        /* Reset refresh counter */
        updateCounter = 0;
    }
}

/* Setup button interrupt handler */
ISR(INT7_vect)
{
    if (mode == MODE_WORK) {
        /* Enter hours setup mode */
        mode = MODE_EDIT_HOURS;
    } else {
        /* Move to minutes or seconds setup mode */
        mode--;
    }

    /* Reset blink mask to make all digits visible */
    newBlinkMask = 0xFF;

    /* Reset blink counter */
    updateCounter = 0x00;

    /* Re-initialize async timer upon exiting setup to ensure a full first second */
    if(mode == MODE_WORK) {
        void initAsyncTimer(); // Forward declaration or call
        initAsyncTimer();
    }
}

/* +1 button interrupt handler */
ISR(INT6_vect)
{
    if (mode == MODE_WORK) {
        return;
    }
    setupIncTime();

    newBlinkMask = 0xFF;
    updateCounter = 0x00;
}

/* Initialize I/O ports */
void initPorts()
{
    /* Set Port D as output */
    DDRD = 0xFF;
    PORTD = 0xFF;

    /* Set PE0-PE5 as outputs, PE6-PE7 as inputs with pull-ups */
    DDRE = PE_INIT_MASK;
    PORTE = 0xFF;
}

/* Initialize asynchronous timer T/C0 */
void initAsyncTimer()
{
    /* Disable T/C0 interrupts */
    TIMSK &= ~((1 << TOIE0) | (1 << OCIE0));

    /* Enable asynchronous mode */
    ASSR |= (1 << AS0);

    /* Reset counter */
    TCNT0 = 0x00;

    /* Set prescaler to 128 */
    TCCR0 |= (1 << CS00) | (1 << CS02);

    /* Wait for registers to update */
    while (ASSR & 0x07);

    /* Enable T/C0 overflow interrupt */
    TIMSK |= (1 << TOIE0);
}

/* Initialize external interrupts */
void initExternalInt()
{
    /* Enable INT6 and INT7 */
    EIMSK |= (1 << INT7) | (1 << INT6);

    /* Trigger interrupts on rising edge */
    EICRB |= (1 << ISC71) | (1 << ISC70) | (1 << ISC61) | (1 << ISC60);
}

/* Initialize T/C1 */
void initTimer1()
{
    /* Set CTC mode, prescaler = 64 */
    TCCR1B |= 0x0B;

    /* Reset counter */
    TCNT1 = 0x00;

    /* Set timer compare limit */
    OCR1A = TIMER_MAX;

    /* Enable T/C1 Compare Match A interrupt */
    TIMSK |= (1 << OCIE1A);
}

/* Initialize all peripherals */
void init()
{
    /* Initialize variables here to avoid ELPM with no RAMPZ error */
    updateCounter = 0;
    blinkMask = 0xFF;
    newBlinkMask = 0xFF;
    time[0] = 0;
    time[1] = 0;
    time[2] = 0;
    lastRefreshedDigit = 0;
    mode = MODE_EDIT_HOURS;
    holdCounter = 0;

    LED_CODE[0] = A|B|C|D|E|F;   // 0x3F -> '0'
    LED_CODE[1] = B|C;           // 0x06 -> '1'
    LED_CODE[2] = A|B|D|E|G;     // 0x5B -> '2'
    LED_CODE[3] = A|B|C|D|G;     // 0x4F -> '3'
    LED_CODE[4] = B|C|F|G;       // 0x66 -> '4'
    LED_CODE[5] = A|C|D|F|G;     // 0x6D -> '5'
    LED_CODE[6] = A|C|D|E|F|G;   // 0x7D -> '6'
    LED_CODE[7] = A|B|C;         // 0x07 -> '7'
    LED_CODE[8] = A|B|C|D|E|F|G; // 0x7F -> '8'
    LED_CODE[9] = A|B|C|D|F|G;   // 0x6F -> '9'

    /* Disable Watchdog */
    WDTCR = 0x10;

    initPorts();
    initAsyncTimer();
    initExternalInt();
    initTimer1();
}

/* Main program loop */
int main()
{
    init();

    /* Enable global interrupts */
    sei();

    /* Infinite loop; operation is interrupt-driven */
    while (1);

    return 0;
}
