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
% Boat, K1, K2, K3, S1, S2, S3
initial_state(left, state([left, left, left], [left, left, left])).
final_state(right, state([right, right, right], [right, right, right])).

%%%%%%%%%%%%%%%%%% Moves %%%%%%%%%%%%%%%%%%
% Knight-Squire in the boat
move(state([Side1, K2, K3], [Side1, S2, S3]), state([Side2, K2, K3], [Side2, S2, S3]), Side1, Side2, knight1+squire1-to-Side2).
move(state([K1, Side1, K3], [S1, Side1, S3]), state([K1, Side2, K3], [S1, Side2, S3]), Side1, Side2, knight2+squire2-to-Side2).
move(state([K1, K2, Side1], [S1, S2, Side1]), state([K1, K2, Side2], [S1, S2, Side2]), Side1, Side2, knight3+squire3-to-Side2).

% Knight-Knight in the boat
move(state([Side1, Side1, K3], [S1, S2, S3]), state([Side2, Side2, K3], [S1, S2, S3]), Side1, Side2, knight1+knight2-to-Side2).
move(state([Side1, K2, Side1], [S1, S2, S3]), state([Side2, K2, Side2], [S1, S2, S3]), Side1, Side2, knight1+knight3-to-Side2).
move(state([K1, Side1, Side1], [S1, S2, S3]), state([K1, Side2, Side2], [S1, S2, S3]), Side1, Side2, knight2+knight3-to-Side2).

% Squire-Squire in the boat
move(state([K1, K2, K3], [Side1, Side1, S3]), state([K1, K2, K3], [Side2, Side2, S3]), Side1, Side2, squire1+squire2-to-Side2).
move(state([K1, K2, K3], [Side1, S2, Side1]), state([K1, K2, K3], [Side2, S2, Side2]), Side1, Side2, squire1+squire3-to-Side2).
move(state([K1, K2, K3], [S1, Side1, Side1]), state([K1, K2, K3], [S1, Side2, Side2]), Side1, Side2, squire2+squire3-to-Side2).

% One passanger in the boat
move(state([Side1, K2, K3], [S1, S2, S3]), state([Side2, K2, K3], [S1, S2, S3]), Side1, Side2, knight1-to-Side2).
move(state([K1, Side1, K3], [S1, S2, S3]), state([K1, Side2, K3], [S1, S2, S3]), Side1, Side2, knight2-to-Side2).
move(state([K1, K2, Side1], [S1, S2, S3]), state([K1, K2, Side2], [S1, S2, S3]), Side1, Side2, knight3-to-Side2).
move(state([K1, K2, K3], [Side1, S2, S3]), state([K1, K2, K3], [Side2, S2, S3]), Side1, Side2, squire1-to-Side2).
move(state([K1, K2, K3], [S1, Side1, S3]), state([K1, K2, K3], [S1, Side2, S3]), Side1, Side2, squire2-to-Side2).
move(state([K1, K2, K3], [S1, S2, Side1]), state([K1, K2, K3], [S1, S2, Side2]), Side1, Side2, squire3-to-Side2).

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

% +From, -To
update_boat(left, right) :- !.
update_boat(right, left).

% -List of Moves from Initial State to Final State
main(Moves) :-
    initial_state(BoatInitSide, state([K1, K2, K3], [S1, S2, S3])),
    dfs(state([K1, K2, K3], [S1, S2, S3]), BoatInitSide, [state([K1, K2, K3], [S1, S2, S3])], Moves),
    print_result(Moves, 1),
    statistics, nl, nl.

/* + Current State, +Boat Place, +Opened States List,
-List of Moves from Current State to Final State */
dfs(State, BoatSide, _, []):-
    final_state(BoatSide, State).
dfs(State, BoatSide, History, [Move | Moves]):-
    update_boat(BoatSide, BoatSide2),
    move(State, State2, BoatSide, BoatSide2, Move),
    \+ member(State2, History),
    legal(State2),
    dfs(State2, BoatSide2, [State2 | History], Moves).

%--------------------------------------------------------------------------------
% Auxiliary predicates
print_result([], _).
print_result([Head | Tail], Number) :-
    format("~d) ~w", [Number, Head]),
    nl,
    NewNumber is Number + 1,
    print_result(Tail, NewNumber). 