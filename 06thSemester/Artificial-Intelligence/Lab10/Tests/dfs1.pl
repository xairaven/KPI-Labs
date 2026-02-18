initial_state(1).
final_state(a).

move(1,0).
move(1,2).
move(1,3).
move(2,a).
move(2,5).
move(2,6).
move(8,10).
move(8,a).
move(6,8).
move(3,33).
move(33,a).
move(33,7).
move(6,a).

main([State | Moves])  :-
    initial_state(State),
    dfs(State, [State], Moves).

dfs(State, _, []) :-
    final_state(State).
dfs(State, History, [Move | Moves]) :-
    move(State, State1),
    \+ member(State1, History),
    Move = State1,
    dfs(State1, [State1|History], Moves).