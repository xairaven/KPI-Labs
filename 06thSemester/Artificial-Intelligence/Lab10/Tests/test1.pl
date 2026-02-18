/* 16
На віконну завіску з малюнком в клітинку сіли 9 мух. Вони розташувалися
так, що ніякі дві мухи не виявилися в одному і тому ж ряду - ні прямому, ні
косому. Через декілька хвилин 3 мухи змінили місця і переповзли в сусідні,
незайняті клітини. Решта 6 мух не рухалися. При цьому всі 9 мух знову
виявилися розміщеними так, що ніяка пара не перебувала в одному прямому
або косому ряду. Які три мухи і куди переповзли?
*/
/* A=1, B=2, C=3, D=4, E=5, F=6, G=7, H=8, I=9*/

% State(9 flies). +FlyId, +X, +Y.
initial_state([fly(1, 1, 6), fly(2, 2, 3), fly(3, 3, 9), fly(4, 4, 4), fly(5, 5, 1), fly(6, 6, 8), fly(7, 7, 2), fly(8, 8, 5), fly(9, 9, 7)]).
% State is final, if it's legal. "3 moves condition" is taken into account in dfs rule
final_state(State) :- legal(State).

% -List of Moves from Initial State to Final State
main(Moves):-
    % Getting initial state
    initial_state(State),
    dfs(State, Moves).

dfs(State, Moves) :-
    History = [State],
    dfs(State, History, 0, Moves).
dfs(State, _, 3, []) :-
    final_state(State).
dfs(State, History, ReplacesCounter, [(TargetIndexFly, DeltaX, DeltaY) | Moves]) :-
    ReplacesCounter \= 3,
    % Getting fly number, Delta X, Delta Y
    member(TargetIndexFly, [1,2,3,4,5,6,7,8,9]),
    member(DeltaX, [-1, 0, 1]),
    member(DeltaY, [-1, 0, 1]),
    % Moving
    move_fly(State, TargetIndexFly, DeltaX, DeltaY, NewState),
    % State must change, so, we write it to the history
    \+ member(NewState, History),
    NewReplacesCounter is ReplacesCounter + 1,
    dfs(NewState, [NewState | History], NewReplacesCounter, Moves).

% All possible states of moving.
move_fly(State, TargetIndexFly, DeltaX, DeltaY, ResultState) :-
    legal_zero_move(DeltaX, DeltaY),
    nth1(TargetIndexFly, State, TargetFly),
    TargetFly = fly(TargetIndexFly, X1, Y1),
    X2 is X1 + DeltaX,
    Y2 is Y1 + DeltaY,
    ResultFly = fly(TargetIndexFly, X2, Y2),
    legal_bounds(ResultFly),
    replace_element_in_list(State, TargetIndexFly, ResultFly, ResultState).

/* %%% Legality check %%% */
/* +State -- State list */
/* ?TargetIndexFly -- Index of fly, that we want to check. */
/* ?CurrentIndexFly -- We want to check all flies except of target. So, it will be loop. Variable for current fly */

% Global legality check
legal(State) :-
    legal_every(State, 1).
legal_every(_, 10).
legal_every(State, CurrentIndex) :-
    % If we start from index 6, for example, no need to check 1 2 3 4 5 again. So, we start from currentindex + 1
    StartIndex is CurrentIndex + 1,
    legal_intersection(State, CurrentIndex, StartIndex),
    legal_diagonal(State, CurrentIndex, StartIndex),
    NextIndex is CurrentIndex + 1,
    legal_every(State, NextIndex).

% In intersection, we also check Rows and Columns.
% Final Case. We are starting from 1, so, if execution is successful on element 10 - there are no intersection.
legal_intersection(_, _, 10).
% If we check a fly under the index of the one that was the ORIGINAL TARGER of the check, we skip it. (Continue)
legal_intersection(State, TargetIndexFly, CurrentIndexFly) :-
    TargetIndexFly = CurrentIndexFly,
    NextIndexFly is CurrentIndexFly + 1,
    legal_intersection(State, TargetIndexFly, NextIndexFly).
% Check all flies, FlyRow1 != FlyRow2, FlyCol1 != FlyCol2
legal_intersection(State, TargetIndexFly, CurrentIndexFly):-
    nth1(TargetIndexFly, State, TargetFly),
    TargetFly = fly(TargetIndexFly, X1, Y1),
    nth1(CurrentIndexFly, State, CurrentFly),
    CurrentFly = fly(CurrentIndexFly, X2, Y2),
    X1 \= X2,
    Y1 \= Y2,
    NextIndexFly is CurrentIndexFly + 1,
    legal_intersection(State, TargetIndexFly, NextIndexFly).

legal_diagonal(_, _, 10).
% If we check a fly under the index of the one that was the ORIGINAL TARGER of the check, we skip it. (Continue)
legal_diagonal(State, TargetIndexFly, CurrentIndexFly) :-
    TargetIndexFly = CurrentIndexFly,
    NextIndexFly is CurrentIndexFly + 1,
    legal_diagonal(State, TargetIndexFly, NextIndexFly).
legal_diagonal(State, TargetIndexFly, CurrentIndexFly):-
    nth1(TargetIndexFly, State, TargetFly),
    TargetFly = fly(TargetIndexFly, X1, Y1),
    nth1(CurrentIndexFly, State, CurrentFly),
    CurrentFly = fly(CurrentIndexFly, X2, Y2),
    DiffX is X1-X2,
    DiffY is Y1-Y2,
    abs(DiffX, AbsX),
    abs(DiffY, AbsY),
    AbsX \= AbsY,
    NextIndexFly is CurrentIndexFly + 1,
    legal_diagonal(State, TargetIndexFly, NextIndexFly).

% Fly cannot move out the table
legal_bounds(fly(_, X, Y)) :-
    X >= 1, X =< 9,
    Y >= 1, Y =< 9.

% Move can't be 0;0
legal_zero_move(X, Y) :-
    (X =\= 0 ; Y =\= 0).

% Auxiliary predicate for replacing one element to another by index
replace_element_in_list(List, Index, NewElem, Result) :-
    nth1(Index, List, _, ListWithoutElement),
    nth1(Index, Result, NewElem, ListWithoutElement).