title lab1.asm

.MODEL tiny
.DATA

; byte
i db 0, 255
i1 db -255         ; must be WORD value
ii db -128, 127

; my values (byte)
i2 db -115
i3 db 115
i4 db -157         ; must be WORD value
i5 db 157

; word
r0 dw -255         
r dw 0, 65535
r1 dw -1, -32768
r2 dw 1, 32767 
r3 dw -65535        ; must be DOUBLE WORD value
r4 dd -65535

; my values (word)
r5 dw -1704
r6 dw 1704
r7 dw -1746
r8 dw 1746

end