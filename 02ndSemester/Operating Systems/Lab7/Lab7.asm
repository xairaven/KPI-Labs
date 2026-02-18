; Task: to calculate
; (b+1)/a, if a > b          A = 2      B = 1      X = 1
; -b, 	   if	a = b 	     A = 5   	B = 5      X = -5
; (a-5)/b, if	a < b	     A = 8   	B = 9      X = 0
; 16-bit

title Lab7.asm					; Title of the program
.model SMALL					; Model of memory for .ехе file
.stack 100h					; Reserving memory for stack
	
code 	segment					
assume	CS:code, DS:data			

begin:	MOV	AX, data			; Initialising DS register 
	MOV	DS, AX				; with the adress of data segment

	MOV	AX, A				; Move the variable A to AX
	CMP	AX, B				; Compare with B
	JG	MORE				; If А > B then jump to (JG, Jump if Greater) on label More
	JE	EQUAL				; If А = B then jump to (JE, Jump if Equals) on label Equal
	JL	LESS				; If А < B then jump to (JL, Jump if Less) on label Less

MORE:						; Label to jump if А > B
	MOV 	AH, 09h				; Output of the message "A > B, then X = (b+1)/a"
	MOV 	DX, OFFSET GREATER_MSG		
	INT 	21h

	MOV	AX, 0				; Move 0 to АХ
	CMP	AX, A				; Compare with А
	JZ	ERROR				; If result = 0, then ZF = 1 - jump to ERROR (JZ, Jump Zero flag)
	
	INC	B				; (b+1) Increment
	MOV	AX, B				; (b+1) Move В to АХ
	CWD					; Word to double word conversion command
	IDIV	A				; (b+1)/a Division with a sign on A
	MOV	X, AX				; The result is written in a variable Х
	JMP	OUTPUT_START			; Go to the "OUTPUT_START" label

EQUAL:						; Label to jump if А = B
	MOV 	AH, 09h				; Output of the message "A = B, then X = -B"
	MOV 	DX, OFFSET EQUALS_MSG	
	INT 	21h	

	NEG	B				; Changing sign of В
	MOV	AX, B				; Move B to АХ
	MOV	X, AX				; The result is written in a variable Х from АХ
	JMP	OUTPUT_START			; Go to the "OUTPUT_START" label

LESS:						; Label to jump if А < B
	MOV 	AH, 09h				; Output of the message "A < B, then X = (a-5)/b"		
	MOV 	DX, OFFSET LESS_MSG	
	INT 	21h
	
	MOV	AX, 0				; Move 0 to АХ
	CMP	AX, B				; Compare with B
	JZ	ERROR				; If result = 0, then ZF = 1 - jump to ERROR (JZ, Jump Zero flag)

	MOV 	AX, A				; (a-5) Move A to АХ
	SBB	AX, 5				; (a-5) Subtract 5 from АХ with "Borrow"
	CWD					; Word to double word conversion command
	IDIV	B				; (a-5)/b Division with a sign on B
	MOV	X, AX				; The result is written in a variable Х
	JMP	OUTPUT_START			; Go to the "OUTPUT_START" label

OUTPUT_START:					; Label, start output
	MOV 	AH, 09h				; Display the message "Result:"
	MOV 	DX, OFFSET RESULT_MSG	
	INT 	21h

	XOR	AX, AX				; Reset the register АХ
	MOV	AX, X				; Returning result
	PUSH	AX				; Store in a stack
	CMP	AX, 0				; Comparing with 0
	JNS 	PLUS				; Test, negative or positive
	MOV	AH, 02h				; The number is negative, we displaying a minus sign
	MOV	DL, '-'
	INT	21h
	POP	AX				; Unload values ​​from the stack
	NEG	AX				; Turning negative into positive

PLUS:	
	XOR 	CX, CX				; Remainder of division counter
	MOV	BX, 10				; Moving the basis of the 10th number system

DIVISION:
	XOR	DX, DX				; The remainder of the division
	DIV	BX				; Divide by 10
	PUSH	DX				; Place in a stack, LIFO
	INC	CX				; Increase the counter
	TEST	AX, AX				; Check if the whole part = 0
	JNZ	DIVISION			; Divide until the whole part is equal 0
	MOV	AH, 02h				; Removing the remainder

OUTPUT:
	POP	DX				; Getting numbers from the stack
	ADD	DL, 30h				; Translating them into ASCII-коди 
	INT	21h				; Interrupting
	LOOP	OUTPUT				; Repeat until the counter is equal 0
	JMP	EXIT				; Exit

ERROR:						; Label, transition is made in case of error
	MOV 	AH, 09h				; Display the message of division by 0
	MOV 	DX, OFFSET ERROR_MSG	
	INT 	21h					
	JMP	EXIT				; Go to the "End of program" label

EXIT:						; Label, transition anyway
	MOV	AH, 4Ch				; Completion of the program		
	MOV     AL, 0				; Code 0 - code of successful completion
	INT	21h			
code	ends

data	segment
	A DW 2
	B DW 1
	X DW ?

	L_BREAK		EQU	0dh, 0ah, '$'	; L_BREAK - constant of CR_LF

	GREATER_MSG	DB	"A > B, then X = (b+1)/a", L_BREAK
	EQUALS_MSG	DB	"A = B, then X = -B", L_BREAK
	LESS_MSG	DB	"A < B, then X = (a-5)/b", L_BREAK

	RESULT_MSG	DB	"Result: ", L_BREAK
	ERROR_MSG	DB	"Error! Division by zero!", L_BREAK
data	ends
	end begin