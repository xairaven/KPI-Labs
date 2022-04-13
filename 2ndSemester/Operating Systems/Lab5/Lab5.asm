title Lab5.asm				; Title of program				
.model SMALL				; Model of memory for exe file
.stack 100h				; Reserving memory for stack

code	segment				
assume 	CS:code, DS:data		; Linking label code with CS and label data with DS

begin:	MOV 	AX, data		; Moving data to DS	
	MOV 	DS, AX

	MOV 	AH, 09h			; Displaying string "Before"
	LEA 	DX, BEFORE
	INT 	21h

	MOV	AH, 40h			; Function code
	MOV	BX, 01h			; Descriptor (01 for display, 00 for files)
	MOV	CX, 0Eh			; Displaying 14 symbols (TEXT length, 0Eh in hex)
	LEA	DX, TEXT	
	INT	21h

	MOV 	AH, 09h			; Carriage return, line feed (or LINE BREAK)
	LEA 	DX, CR_LF
	INT 	21h
	MOV 	AH, 09h
	LEA 	DX, CR_LF
	INT 	21h

	MOV 	AL, SYMBOL		; Moving "*" to AL

	MOV	CX, 4			; Loop with 4 iterations
	MOV	BX, 3			; Index 3, because we need exchange 9001 in TEXT to "*"
L1:					; Loop start
	MOV	TEXT[BX], AL		; Moving "*" to TEXT symbol with index BX (3, 4, 5, 6)
	INC	BX			; Incrementing BX (3, 4, 5, 6)
LOOP L1					; Jumping to start of the loop


	MOV	CX, 2			; Second loop with 2 iterations
	MOV	BX, 9			; Index 9, because we need exchange 12 in TEXT to "*"
L2:					; Second loop start
	MOV	TEXT[BX], AL		; Moving "*" to TEXT symbol with index BX (9, 10)
	INC	BX			; Incrementing BX (9, 10)
LOOP L2					; Jumping to start of the loop
	MOV 	TEXT[12], AL		; Changing 12th symbol in TEXT by the traditional way
	MOV 	TEXT[13], AL		; Changing 13th symbol in TEXT by the traditional way

	MOV 	AH, 09h			; Displaying string "After"			
	LEA 	DX, AFTER
	INT 	21h

	MOV	AH, 40h			; Function code
	MOV	BX, 01h			; Descriptor (01 for display, 00 for files)
	MOV	CX, 0Eh			; Displaying 14 symbols (TEXT length, 0Eh in hex)
	LEA	DX, TEXT
	INT	21h

	MOV 	AH, 09h			; Carriage return, line feed (or LINE BREAK)
	LEA 	DX, CR_LF
	INT 	21h

	MOV	AH, 4Ch			; Exit function		
	MOV     Al, 0			; Code 0
	INT	21h
code 	ends

data	segment
	L_BREAK	EQU	0dh, 0ah, '$'	; L_BREAK - constant of CR_LF
	CR_LF 	DB 	0dh, 0ah, '$'
	BEFORE	DB	"Before:", L_BREAK
	AFTER	DB	"After:", L_BREAK
	TEXT	DB	"ISO9001GR12H45", L_BREAK
	SYMBOL	DB	"*"
data 	ends
	end begin