% Реалізація пошуку за стратегією hill-climbing з підрахунком
% розкритих станів
% - List of Moves from Initial State to Final State,
% - List of Open States,
% - Number of Open States
main(Moves, ListOfStates, StatesAmount):-
    initial_state(InitialScore, InitialState),
    hill_climb(InitialScore-InitialState, [InitialState], Moves),
    findall(State, recorded(states_db, State), ListOfStates),
    length(ListOfStates, StatesAmount).
    
/* + Current State, + Opened States List,
- List of Moves from Current State to Final State, */
hill_climb(State, _, []) :-
    final_state(FinalScore, FinalState),
    State = FinalScore-FinalState.
% final_state(State).
hill_climb(_-state(OldBoat, Wolf, Goat, Cabbage), History, [Move | Moves]):-
    update_boat_side(OldBoat, Boat),
    setof(HeuristicValue-state(Boat, WolfTemp, GoatTemp, CabbageTemp)-MoveDescription, (
        move(state(OldBoat, Wolf, Goat, Cabbage), state(Boat, WolfTemp, GoatTemp, CabbageTemp), MoveDescription),
        legal(state(Boat, WolfTemp, GoatTemp, CabbageTemp)),
        calculate_heuristic(state(Boat, WolfTemp, GoatTemp, CabbageTemp), MoveDescription, HeuristicValue), HeuristicValue<100)
        , PossibleMoves),
    member(CurrentHeuristicValue-state(Boat, WolfNext, GoatNext, CabbageNext)-Move, PossibleMoves),
    recorda(states_db, state(Boat, WolfNext, GoatNext, CabbageNext)),
    hill_climb(CurrentHeuristicValue-state(Boat, WolfNext, GoatNext, CabbageNext), 
        [state(Boat, WolfNext, GoatNext, CabbageNext) | History], Moves).
    
% Представлення задачі фермера
% state(? Boat Place, ? Wolf Place, ? Goat Place, ? Cabbage Place )
initial_state(100, state(left,left,left,left)).
final_state(0, state(right,right,right,right)).

% +Current State, -Next State
move(state(BoatSide, BoatSide, Goat, Cabbage), state(BoatSideNext, BoatSideNext, Goat, Cabbage), wolf-BoatSideNext).
move(state(BoatSide, Wolf, BoatSide, Cabbage), state(BoatSideNext, Wolf, BoatSideNext, Cabbage), goat-BoatSideNext).
move(state(BoatSide, Wolf, Goat, BoatSide), state(BoatSideNext, Wolf, Goat, BoatSideNext), cabbage-BoatSideNext).
move(state(_, Wolf, Goat, Cabbage), state(BoatSideNext, Wolf, Goat, Cabbage), alone-BoatSideNext).

% + State
% BoatSide, WolfSide, GoatSide, CabbageSide
% If Wolf and Goat on the same side without boat (farmer)
legal(state(BoatSide, AnySide, AnySide, _)):-
    BoatSide\=AnySide,
    !,fail.
% If Cabbage and Goat on the same side without boat (farmer)
legal(state(BoatSide, _, AnySide, AnySide)):-
    BoatSide \= AnySide,
    !,fail.
% Else - success
legal(_).
    
% +From, - To
update_boat_side(left, right):- !.
update_boat_side(right, left).

% +State, + Cargo-Boat Direction, - State Value
calculate_heuristic(state(_, Wolf, Goat, Cabbage), Cargo-Direction, HeuristicValue):-
    side_to_int(Wolf, WolfValue),
    side_to_int(Goat, GoatValue),
    side_to_int(Cabbage, CabbageValue),
    heuristic_formula(Cargo, Direction, WolfValue, GoatValue, CabbageValue, MoveValue),
    HeuristicValue is (WolfValue+GoatValue+CabbageValue)*MoveValue,
    !.

% Convert boat side to int
side_to_int(left, 1).
side_to_int(right, 0).

% Formulas for Heuristic Score
heuristic_formula(alone, left, WolfValue, _, CabbageValue, abs(WolfValue-CabbageValue)*100+1) :- !. % <--
heuristic_formula(goat, right, WolfValue, _, CabbageValue, abs(WolfValue-CabbageValue)*100+1) :- !. % -->
heuristic_formula(goat, left, WolfValue, _, CabbageValue, (1-abs(WolfValue-CabbageValue))*100+1) :- !. % <--
heuristic_formula(wolf, right, _, GoatValue, CabbageValue, (1-abs(GoatValue-CabbageValue))*100+1) :- !. % -->
heuristic_formula(cabbage, right, WolfValue, GoatValue, _, (1-abs(GoatValue-WolfValue))*100+1) :- !. % -->
heuristic_formula(_,_,_,_,_,101). % ((wolf/cabbage <--)/(alone -->))