/* 19
Three knights, each accompanied by a squire, gathered on the bank of the river, 
intending to cross to the other side. They managed to find a small two-seater boat, 
and the crossing would be easy. But all the squires flatly refused to remain in the 
society of unfamiliar knights without their masters. And yet the crossing took place, 
all six people crossed to the other shore with the help of one two-seater boat. 
At the same time, the condition insisted on by the squires was observed. How was it done?

Три лицаря, кожен у супроводі зброєносця, з'їхалися на березі річки, маючи
намір переправитися на іншу сторону. Їм вдалося знайти маленький
двомісний човен, і переправа сталася б легко. Але всі зброєносці навідріз
відмовилися залишатися в суспільстві незнайомих лицарів без своїх
господарів. І все ж переправа відбулася, всі шестеро людей перебралися на
інший берег за допомогою одного двомісного човна. При цьому
дотримувалася умова, на якій наполягали зброєносці. Як це було зроблено?
*/

% Knight - K
% Squire - S (Armor Bearer)
% Places of:
% Score, Boat, K1, K2, K3, S1, S2, S3
initial_state(100, left, state([left, left, left], [left, left, left])).
final_state(0, right, state([right, right, right], [right, right, right])).

main(Moves):-
    initial_state(InitScore, InitBoatSide, InitState),
    hill_climb(InitScore-InitState, InitBoatSide, [InitState], Moves),
    print_result(Moves, 1),
    statistics, nl, nl.
    
main(Moves, ListOfStates, StatesAmount):-
    initial_state(InitScore, InitBoatSide, InitState),
    hill_climb(InitScore-InitState, InitBoatSide, [InitState], Moves),
    findall(State, recorded(states_db, State), ListOfStates),
    length(ListOfStates, StatesAmount),
    print_result(Moves, 1).

%%%%%%%%%%%%%%%%%% Hill-Climb %%%%%%%%%%%%%%%%%%
% +Score-State, +BoatSide, +Opened States List, -List of Moves from Current State to Final State, */
hill_climb(StateComplex, _, _, []) :-
    final_state(FinalScore, _, FinalState),
    StateComplex = FinalScore-FinalState.
hill_climb(_-State, BoatSide, History, [NewMoveDescription | Moves]):-
    switch_boat_direction(BoatSide, BoatSideNew),
    get_possible_moves(State, BoatSide, BoatSideNew, PossibleMoves),
    member(CurrentHeuristicValue-NewMoveDescription-CurrentState, PossibleMoves),
    \+ member(CurrentState, History),
    legal(CurrentState),
    recorda(states_db, CurrentState),
    hill_climb(CurrentHeuristicValue-CurrentState, BoatSideNew, [CurrentState | History], Moves).

get_possible_moves(State, BoatSide, BoatSideNew, PossibleMoves) :-
    findall(HeuristicValue-MoveDescription-NewState, (
        move(State, NewState, BoatSide, BoatSideNew, MoveDescription),
        switch_state_to_int(NewState, IntState),
        calc_heuristic(MoveDescription, IntState, HeuristicValue),
        HeuristicValue < 100
        ), PossibleMovesNotSorted),
    sort(PossibleMovesNotSorted, PossibleMoves).

%%%%%%%%%%%%%%%%%% Moves %%%%%%%%%%%%%%%%%%
% +InitialState, -NewState, +InitialSide, -NewSide, -MoveDescription
% Knight-Squire in the boat
move(state([Side1, K2, K3], [Side1, S2, S3]), state([Side2, K2, K3], [Side2, S2, S3]), Side1, Side2, knight-1+squire-1-to-Side2).
move(state([K1, Side1, K3], [S1, Side1, S3]), state([K1, Side2, K3], [S1, Side2, S3]), Side1, Side2, knight-2+squire-2-to-Side2).
move(state([K1, K2, Side1], [S1, S2, Side1]), state([K1, K2, Side2], [S1, S2, Side2]), Side1, Side2, knight-3+squire-3-to-Side2).

% Knight-Knight in the boat
move(state([Side1, Side1, K3], [S1, S2, S3]), state([Side2, Side2, K3], [S1, S2, S3]), Side1, Side2, knight-1+knight-2-to-Side2).
move(state([Side1, K2, Side1], [S1, S2, S3]), state([Side2, K2, Side2], [S1, S2, S3]), Side1, Side2, knight-1+knight-3-to-Side2).
move(state([K1, Side1, Side1], [S1, S2, S3]), state([K1, Side2, Side2], [S1, S2, S3]), Side1, Side2, knight-2+knight-3-to-Side2).

% Squire-Squire in the boat
move(state([K1, K2, K3], [Side1, Side1, S3]), state([K1, K2, K3], [Side2, Side2, S3]), Side1, Side2, squire-1+squire-2-to-Side2).
move(state([K1, K2, K3], [Side1, S2, Side1]), state([K1, K2, K3], [Side2, S2, Side2]), Side1, Side2, squire-1+squire-3-to-Side2).
move(state([K1, K2, K3], [S1, Side1, Side1]), state([K1, K2, K3], [S1, Side2, Side2]), Side1, Side2, squire-2+squire-3-to-Side2).

% One passanger in the boat
move(state([Side1, K2, K3], [S1, S2, S3]), state([Side2, K2, K3], [S1, S2, S3]), Side1, Side2, knight-1-to-Side2).
move(state([K1, Side1, K3], [S1, S2, S3]), state([K1, Side2, K3], [S1, S2, S3]), Side1, Side2, knight-2-to-Side2).
move(state([K1, K2, Side1], [S1, S2, S3]), state([K1, K2, Side2], [S1, S2, S3]), Side1, Side2, knight-3-to-Side2).
move(state([K1, K2, K3], [Side1, S2, S3]), state([K1, K2, K3], [Side2, S2, S3]), Side1, Side2, squire-1-to-Side2).
move(state([K1, K2, K3], [S1, Side1, S3]), state([K1, K2, K3], [S1, Side2, S3]), Side1, Side2, squire-2-to-Side2).
move(state([K1, K2, K3], [S1, S2, Side1]), state([K1, K2, K3], [S1, S2, Side2]), Side1, Side2, squire-3-to-Side2).


%%%%%%%%%%%%%%%%%% Checking for legality  %%%%%%%%%%%%%%%%%%
% +State
legal(state([K1, K2, _], [S1, _, _])) :-
    S1 \= K1,
    S1 = K2, !, fail.
legal(state([K1, _, K3], [S1, _, _])) :-
    S1 \= K1,
    S1 = K3, !, fail.
legal(state([K1, K2, _], [_, S2, _])) :-
    S2 \= K2,
    S2 = K1, !, fail.
legal(state([_, K2, K3], [_, S2, _])) :-
    S2 \= K2,
    S2 = K3, !, fail.
legal(state([K1, _, K3], [_, _, S3])) :-
    S3 \= K3,
    S3 = K1, !, fail.
legal(state([_, K2, K3], [_, _, S3])) :-
    S3 \= K3,
    S3 = K2, !, fail.
legal(_).


%%%%%%%%%%%%%%%%%% Heuristic formulas %%%%%%%%%%%%%%%%%%
calc_heuristic(MoveDescription, IntState, HeuristicValue) :-
    heuristic_formula(MoveDescription, IntState, HeuristicValue), !.

% +MoveDescription, +State, -Value
heuristic_formula(_, state([K1, K2, K3], [S1, S2, S3]), 0) :-
    Sum is K1 + K2 + K3 + S1 + S2 + S3,
    Sum = 6.
heuristic_formula(_, state([K1, K2, K3], [S1, S2, S3]), Value) :-
    % Calculate distance to goal
    DistanceToGoal is K1 + K2 + K3 + S1 + S2 + S3,
    % Evaluate progress towards the goal state
    Progress is 6 - DistanceToGoal,
    % Calculate heuristic value
    Value is Progress * 10.

%%%%%%%%%%%%%%%%%% Auxiliary predicates %%%%%%%%%%%%%%%%%%
% +StringState, -IntState
switch_state_to_int(State, Result) :-
    State = state([K1, K2, K3], [S1, S2, S3]),
    side_to_int(K1, KN1), side_to_int(K2, KN2), side_to_int(K3, KN3),
    side_to_int(S1, SN1), side_to_int(S2, SN2), side_to_int(S3, SN3),
    Result = state([KN1, KN2, KN3], [SN1, SN2, SN3]).

% +String, -Int
side_to_int(left, 0).
side_to_int(right, 1).

% +From, -To
switch_boat_direction(left, right) :- !.
switch_boat_direction(right, left).

% +List, +StartNumber
print_result([], _).
print_result([Head | Tail], Number) :-
    format("~d) ~w", [Number, Head]),
    nl,
    NewNumber is Number + 1,
    print_result(Tail, NewNumber).

%%%%%%%%%%%%%%%%%% Predicates for tests %%%%%%%%%%%%%%%%%%
% Print possible moves list (for specific state)
% +State, +OldBoatSide, +NewBoatSide
test_pm(State, OldBoat, Boat) :-
    get_possible_moves(State, OldBoat, Boat, PossibleMoves),
    print_result(PossibleMoves, 1).