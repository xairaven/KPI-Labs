.nolist
.include "m8def.inc"
.list

.def Counter = R16
.def Temp = R17

.CSEG
.ORG 0x0000
RJMP Reset

Reset:
    ; Initialize stack pointer
    LDI Temp, low(RAMEND)
    OUT SPL, Temp
    LDI Temp, high(RAMEND)
    OUT SPH, Temp

Init:
    ; Configure Port D as output
    LDI Temp, 0xFF
    OUT DDRD, Temp
    OUT PORTD, Temp
    
    ; Clear counter
    CLR Counter

Loop:
    ; Load base address of sine table into Z register
    LDI ZL, low(Sinus*2)
    LDI ZH, high(Sinus*2)

    ; Add current counter value to address
    ADD ZL, Counter
    CLR Temp
    ADC ZH, Temp

    ; Read sine value from flash memory
    LPM Temp, Z

    ; Output value to DAC (Port D)
    OUT PORTD, Temp

    ; Increment counter (auto-wraps to 0 after 255)
    INC Counter

    ; Repeat loop
    RJMP Loop

; Sine wave values table (256 values)
Sinus:
.DB 64,65,67,68,70,72,73,75
.DB 76,78,79,81,82,84,85,87
.DB 88,90,91,92,94,95,97,98
.DB 99,100,102,103,104,105,107,108
.DB 109,110,111,112,113,114,115,116
.DB 117,118,118,119,120,121,121,122
.DB 123,123,124,124,125,125,126,126
.DB 126,127,127,127,127,127,127,127
.DB 128,127,127,127,127,127,127,127
.DB 126,126,126,125,125,124,124,123
.DB 123,122,121,121,120,119,118,118
.DB 117,116,115,114,113,112,111,110
.DB 109,108,107,105,104,103,102,100
.DB 99,98,97,95,94,92,91,90
.DB 88,87,85,84,82,81,79,78
.DB 76,75,73,72,70,68,67,65
.DB 64,62,61,59,58,56,54,53
.DB 51,50,48,47,45,44,42,41
.DB 39,38,36,35,34,32,31,30
.DB 28,27,26,25,23,22,21,20
.DB 19,18,17,15,14,13,13,12
.DB 11,10,9,8,8,7,6,5
.DB 5,4,4,3,3,2,2,2
.DB 1,1,1,0,0,0,0,0
.DB 0,0,0,0,0,0,1,1
.DB 1,2,2,2,3,3,4,4
.DB 5,5,6,7,8,8,9,10
.DB 11,12,13,13,14,15,17,18
.DB 19,20,21,22,23,25,26,27
.DB 28,30,31,32,34,35,36,38
.DB 39,41,42,44,45,47,48,50
.DB 51,53,54,56,58,59,61,62