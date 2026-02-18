% Реалізація пошуку за стратегією
% hill-climbing
% -List of Moves from Initial Vertex to
% Final Vertex
main([State|Moves]):-
    initial_state(State),
    hill_climb(100-State,[State],Moves).
    
/* + Current State, + Open States List,
- List of Moves from Current State to Final State
*/
hill_climb(0-_,_,[]).
% final_state(State).
hill_climb(_-State,History,[Move1|Moves]):-
    setof(Eval-StateI-MoveI,
     ( move(State,MoveI),
     update(State,MoveI,StateI),
     legal(StateI)
     %,  val_state(StateI,Eval) 
     ),
     NextMoves),
    member(Val-State1-Move1,NextMoves),
    \+ member(State1,History),
    hill_climb(Val-State1,[State1|History],Moves).

% Представлення задачі
initial_state(a).
% final_state(x).
update(_,B,B).
legal(_).
move(a,b).
move(a,c).
move(a,d).
move(c,k).
move(c,g).
move(d,e).
move(d,f).
move(k,l).
move(k,m).
move(g,n).
move(g,o).
move(f,p).
move(f,x).
move(m,x).