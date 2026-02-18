; Task - to calculate (12/c - d*4 + 73)/(a*a + 1)
; 8-bit

title Lab6.asm					; Title of the program
.model SMALL					; Model of memory for .ехе file
.stack 100h					; Reserving memory for stack
	
code 	segment
assume	CS:code, DS:data

begin:	MOV 	AX, data			; Initialising DS register  
	MOV 	DS, AX				; with the adress of data segment
	MOV	AL, 0				; Move the value 0 to the register AL
	CMP	AL, C				; We compare it with C. If C = 0 - then
	JE	ERROR				; we pass to a label ERROR (JE - JUMP EQUAL)
						; otherwise we go further.
	MOV 	AL, 12				; (12/c) Moving 12 to AL.
	CBW					; Byte to word conversion command
	IDIV	C				; (12/c) Division with a sign on C
	PUSH	AX				; (12/c) The result is temporarily moved to the stack
	MOV	AL, 4				; (d*4) Moving 4 to АL
	CBW					; Byte to word conversion command
	IMUL	D				; (d*4) Multiply by D
	MOV	BL, AL				; (d*4) Temporarily move the result to BL
	POP	AX				; (12/c) Get the value from the stack to АХ
	SBB	AL, BL				; (12/c - d*4) Subtraction with borrow
	ADC	AL, 73				; (12/c - d*4 + 73) Adding with borrow
	PUSH	AX				; (12/c - d*4 + 73) Moving to stack
	MOV	AL, A				; (a*a) Transfer the value of variable A to AL
	CWD					; Byte to word conversion command
	IMUL	A				; (a*a) Multiplication
	INC	AL				; (a*a + 1) Increment	
	MOV	BL, AL				; (a*a + 1) Temporarily move the result to ВL	
	POP	AX				; (12/c - d*4 + 73) Get the value from the stack to АХ
	CWD					; Byte to word conversion command
	IDIV	BL				; (The whole formula) Divide by BL
	MOV	X, AL				; The result is written in a variable L
	JMP	OUTPUT_START			; Go to the "Output_Start" label		

OUTPUT_START:					; Label, start output
	MOV 	AH, 09h				; Display the message "Result:"
	MOV 	DX, OFFSET SUCCESS_MSG	
	INT 	21h

	XOR	AX, AX				; Reset the register АХ
	MOV	AL, X				; Returning result
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
	A DB 1
	C DB 12
	D DB 5
	X DB ?
	
	L_BREAK		EQU	0dh, 0ah, '$'	; L_BREAK - constant of CR_LF
	SUCCESS_MSG	DB	"Result: ", L_BREAK
	ERROR_MSG	DB	"Error! Division by zero!", L_BREAK
data	ends
	end begin