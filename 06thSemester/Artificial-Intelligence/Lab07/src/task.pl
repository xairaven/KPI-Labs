% 19
% Given a square matrix of unknown size. 
% Get a matrix in which the halves of the main and side 
% diagonals located to the right of the center are interchanged.

matrix(1, [
    [1, 2, 3, 4, 5],
    [6, 7, 8, 9, 1],
    [2, 3, 4, 5, 6],
    [7, 8, 9, 1, 2],
    [3, 4, 5, 6, 7]
    ]).

matrix(2, [
    [0, 8, 1, 9, 4, 6],
    [1, 4, 1, 6, 9, 5],
    [9, 4, 9, 3, 3, 0],
    [4, 1, 9, 5, 8, 3],
    [6, 6, 0, 9, 8, 4],
    [7, 1, 4, 6, 4, 9]
    ]).

matrix(3, [
    [1, 2],
    [3, 4]
    ]).

matrix(4, [[1]]).

matrix(5, [[]]).

matrix(6, []).

task(K, ResultMatrix) :-
    matrix(K, Matrix),
    length(Matrix, MatrixOrder),
    BoundaryIndex is MatrixOrder // 2,
    replace_elements_in_diagonals(Matrix, MatrixOrder, BoundaryIndex, MatrixOrder, ResultMatrix),
    print_matrix(ResultMatrix).
    
replace_element_in_list(List, Index, NewElem, Result) :-
    nth1(Index, List, _, ListWithoutElement),
    nth1(Index, Result, NewElem, ListWithoutElement).

replace_elements_in_diagonals(Matrix, MatrixOrder, BoundaryIndex, Index, Result) :-
    Index > BoundaryIndex,
    FirstIndex is MatrixOrder-Index+1,
    LastIndex is Index,
    nth1(FirstIndex, Matrix, FirstSwapRow),
    nth1(LastIndex, Matrix, LastSwapRow),

    nth1(Index, FirstSwapRow, FirstElement),
    nth1(Index, LastSwapRow, SecondElement),
    replace_element_in_list(FirstSwapRow, Index, SecondElement, NewFirstRow),
    replace_element_in_list(LastSwapRow, Index, FirstElement, NewLastRow),

    replace_element_in_list(Matrix, FirstIndex, NewFirstRow, TempNewMatrix),
    replace_element_in_list(TempNewMatrix, LastIndex, NewLastRow, NewMatrix),

    NewIndex is Index - 1,
    replace_elements_in_diagonals(NewMatrix, MatrixOrder, BoundaryIndex, NewIndex, Result).
replace_elements_in_diagonals(Matrix, _, BoundaryIndex, BoundaryIndex, Matrix).

print_matrix([]).
print_matrix([Row|Rest]) :-
    write(Row), nl,
    print_matrix(Rest).