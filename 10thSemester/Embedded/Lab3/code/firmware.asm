.nolist
.include "m32def.inc"
.list

.equ fck = 8000000
.equ BAUD = 9600
.equ UBRR_value = (fck/(BAUD*16))-1

.cseg
.org 0

Start:
    ; Initialize Stack Pointer
    ldi R16, high(RAMEND)
    out SPH, R16
    ldi R16, low(RAMEND)
    out SPL, R16

    rcall init_USART

    ; Configure PORTC as output for LEDs
    ldi R16, 0xFF
    out DDRC, R16

Main_Loop:
    ; Receive data from terminal
    rcall USART_receive
    ; Output received data to LEDs
    out PORTC, R16

    ; Configure PORTA as input for switches
    ldi R16, 0x00
    out DDRA, R16
    
    ; Enable internal pull-up resistors for PORTA
    ldi R16, 0xFF
    out PORTA, R16

    ; Read switches state
    in R16, PINA
    ; Send data to terminal
    rcall USART_send

    rjmp Main_Loop

init_USART:
    ; Set baud rate
    ldi R16, high(UBRR_value)
    out UBRRH, R16
    ldi R16, low(UBRR_value)
    out UBRRL, R16

    ; Enable receiver and transmitter
    ldi R16, (1<<RXEN)|(1<<TXEN)|(0<<RXCIE)|(0<<TXCIE)|(0<<UDRIE)
    out UCSRB, R16

    ; Set frame format: 8 data bits, no parity, 1 stop bit
    ldi R16, (1<<URSEL)|(1<<UCSZ1)|(1<<UCSZ0)
    out UCSRC, R16
    ret

USART_send:
    ; Wait for empty transmit buffer
sending:
    sbis UCSRA, UDRE
    rjmp sending
    ; Put data into buffer, sends the data
    out UDR, R16
    ret

USART_receive:
    ; Wait for data to be received
receiving:
    sbis UCSRA, RXC
    rjmp receiving
    ; Get and return received data from buffer
    in R16, UDR
    ret