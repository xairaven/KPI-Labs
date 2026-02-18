% Street - list of houses
% House(Color, Occupation, Bird, FavDrink, FavTransport)

task(Owner) :- puzzle(Street), print_street(Street, 1), nl, find_desired_house(Street, Owner).

puzzle(Street) :-
    Street = [_, _, _, _, _],
    
    member(house(_, poet, _, _, bicycle), Street),
    member(house(_, writer, _, coffee, _), Street),
    member(house(_, critic, _, _, _), Street),
    member(house(_, journalist, parrot, _, _), Street),
    member(house(red, editor, _, _, _), Street),
    member(house(_, _, bullfinch, milk, _), Street),
    member(house(yellow, _, _, tea, _), Street),
    member(house(_, _, _, apple_juice, car), Street),

    % There is a magpie in some house
    member(house(_, _, magpie, _, _), Street),

    % Unknown Transport
    member(house(_, _, _, _, 'UNKNOWN_TRANSPORT'), Street),

    nth1(1, Street, house(_, critic, _, _, _)),
    nth1(2, Street, house(blue, _, _, _, _)),
    nth1(3, Street, house(_, _, _, _, motorcycle)),

    sublist([house(white, _, _, _, _), house(green, _, _, _, walking)], Street),
    chickadee_owner(Street),
    tea_lover(Street).

chickadee_owner(Street) :-
    sublist([house(_, _, chickadee, _, _), house(_, _, _, cocoa, _)], Street) ;
    sublist([house(_, _, _, cocoa, _), house(_, _, chickadee, _, _)], Street).

tea_lover(Street) :-
    sublist([house(_, _, _, tea, _), house(_, _, canary, _, _)], Street) ;
    sublist([house(_, _, canary, _, _), house(_, _, _, tea, _)], Street).

%--------------------------------------------------------------------------------
% Task result output predicate:

is_magpie_owner(house(_, _, magpie, _, _)).
find_desired_house(Street, DesiredHouse) :-
    member(DesiredHouse, Street),
    is_magpie_owner(DesiredHouse).

%--------------------------------------------------------------------------------
% Auxiliary predicates

prefix(P, L) :- append(P, _, L).
suffix(S, L) :- append(_, S, L).
sublist(SubL, L) :- suffix(S, L), prefix(SubL, S).

%--------------------------------------------------------------------------------
% Predicate for printing street

print_street([], _).
print_street([House|Rest], Number) :-
    House =.. [house|Args],
    format("~w) ~w, ~w, ~w, ~w, ~w~n", [Number|Args]),
    NewNumber is Number + 1,
    print_street(Rest, NewNumber).