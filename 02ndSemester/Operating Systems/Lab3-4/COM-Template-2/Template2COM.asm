CODE 	SEGMENT
ASSUME	CS:CODE, DS:CODE, SS:CODE
ORG	100h
START:
	mov	AH, 09h
	mov     DX, offset msg
	int 	21h
	mov	AH, 4Ch		
	mov     Al, 0			
	int	21h
msg 	db	"Hello World!", 0Dh, 0Ah, '$'
CODE 	ENDS
END 	START