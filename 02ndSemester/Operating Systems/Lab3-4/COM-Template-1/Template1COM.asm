	.MODEL TINY
	.CODE
	ORG 	100h
program:
	mov	AH, 09h
	mov     DX, offset msg
	int 	21h
	mov	AH, 4Ch		
	mov     Al, 0			
	int	21h
	RET
msg 	db	"Hello World!", 0Dh, 0Ah, '$'
	end	program
