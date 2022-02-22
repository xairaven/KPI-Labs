title Lab2.asm
.model SMALL

text	segment			
assume 	CS:text, DS:data	
					
begin:	mov	AX, data		
	mov	DS, AX		
	mov	AH, 09h		
	mov	DX, offset mesg
	int 	21h			
	mov	AH, 4Ch		
	mov     Al, 0			
	int	21h			
text	ends				

data	segment			
mesg	db "NACHINAEM ! $"
data	ends								
	end begin