STK	SEGMENT STACK
	DB	256 DUP (?)
STK	ENDS

DATA	SEGMENT
	msg 	db	"Hello World!", 0Dh, 0Ah, '$'
DATA 	ENDS

CODE 	SEGMENT
ASSUME	CS:CODE, DS:DATA, SS:STK
START:
	mov	AX, DATA
	mov	DS, AX
	mov	AH, 09h		
	mov	DX, offset msg
	int 	21h			
	mov	AH, 4Ch		
	mov     Al, 0			
	int	21h
CODE	ENDS
END	START