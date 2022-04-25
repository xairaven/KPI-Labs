TITLE	ASCOM	COM-TEMPLATE-THREE
CODESEG SEGMENT	PARA	'Code'
ASSUME 	CS:CODESEG, DS:CODESEG, SS:CODESEG, ES:CODESEG
ORG	100h
Begin:	JMP MAIN
msg 	db	"Hello World!", 0Dh, 0Ah, '$'
MAIN	PROC	NEAR
	mov	AH, 09h
	mov     DX, offset msg
	int 	21h
	mov	AH, 4Ch		
	mov     Al, 0			
	int	21h
MAIN	ENDP
CODESEG	ENDS
	END BEGIN