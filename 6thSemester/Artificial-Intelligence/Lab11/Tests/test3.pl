foo(a, b, c).
foo(a, b, d).
foo(b, c, e).
foo(b, c, f).
foo(c, c, g).

/*
2 ?- bagof(C, foo(A, B, C), Cs).
A = a,      
B = b,      
Cs = [c, d] ;
A = b,
B = c,
Cs = [e, f] ;
A = B, B = c,
Cs = [g].

3 ?- bagof(C, A^foo(A, B, C), Cs). 
B = b,
Cs = [c, d] ;
B = c,
Cs = [e, f, g].

4 ?- bagof(C, A^B^foo(A, B, C), Cs). 
Cs = [c, d, e, f, g].
*/