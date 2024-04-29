/***** Wolf-Goat-Cabbage Problem *****/
% state(?boat Place, ?wolf Place, ?goat Place,?cabbage Place)
initial_state(state(left, left, left, left)).
final_state(state(right, right, right, right)).

% +Current State, -Next State, +From, +To, -Move
move(state(B,B,G,C), state(B1,B1,G,C), B, B1, wolf-to-B1).
move(state(B,W,B,C), state(B1,W,B1,C), B, B1, goat-to-B1).
move(state(B,W,G,B), state(B1,W,G,B1), B, B1, cabbage-to-B1).
move(state(_,W,G,C), state(B1,W,G,C), _,  B1, alone-to-B1).

% +State
legal(state(F, X, X, _)):-
    F \= X,
    !, fail.
legal(state(F, _, X, X)):-
    F \= X,
    !, fail.
legal(_).

% +From, -To
update_boat(left, right) :- !.
update_boat(right, left).

% -List of Moves from Initial State to Final State
main(Moves):-
    initial_state(state(B,W,G,C)),
    dfs(state(B,W,G,C), B, [state(B,W,G,C)], Moves),
    statistics.

/* +Current State, +Boat Place, +Opened States List,
-List of Moves from Current State to Final State */
dfs(State, _, _, []):-
    final_state(State).
dfs(S, B, History, [Move | Moves]):-
    update_boat(B, B1),
    move(S, S1, B, B1, Move),
    legal(S1),
    \+ member(S1, History),
    dfs(S1, B1, [S1 | History], Moves).