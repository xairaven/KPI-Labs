; Using macro library (MyMac.mac)

title Lab8.asm						; Title of the program
.model SMALL						; Model of memory for .ехе file
.stack 100h						; Reserving memory for stack
include MyMac.mac					; IMPORTING MACROS
	
code 	segment						; Code segment :)
assume	CS:code, DS:data				; Connecting segment registers with names of registers

begin:	INIT						; INITIALIZATION MACRO 
	OFFSET_CALC 	2, LEN_ARR, offset_value	; OFFSET CALCULATING MACRO
	PRINT_STR	RESULT_MSG			; STRING PRINTING MACRO
	TASK		LEN_ARR, offset_value, matrix	; MACRO FOR THE TASK
	EXIT_LAB					; ENDING PROGRAM MACRO
code	ends						; End of code segment

data	segment						; Data segment
	LEN_ARR EQU 5					; Symbolic value for line length

	L_BREAK	EQU 0dh, 0ah, '$'			; Symbolic value for carriage transfer
	SPACE DB ' '					; Symbol " "
	CR_LF DB L_BREAK				; Carriage Return, Line Feed
	RESULT_MSG DB "Result:", L_BREAK		; String for result
	
	offset_value DW ?				; Offset in two-dimensional array between rows
	COUNTER	DW 1					; Counter to fill the array with numbers from 1 to 25
	i DW 0						; Two-dimensional array row counter
	j DW 0						; Two-dimensional array column counter
	matrix dw LEN_ARR dup (LEN_ARR dup(?))		; Two-dimensional array length 5х5
data	ends						; End of data segment
	end begin					; End of program