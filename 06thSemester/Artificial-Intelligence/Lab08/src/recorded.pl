% 19
% RECORDS MECHANISM
% Using the given center and radiuses of two concentric circles, 
% find all points belonging to the ring formed by the given circles.

%?Point ID, ?X, ?Y
db_filling:-
    recorded(db, point(_,_,_)), !.

db_filling:-
    recordz(db, point(a, 4, 4)),
    recordz(db, point(b, 2, 5)),
    recordz(db, point(c, 2, 8)),
    recordz(db, point(d, 5, 1)),
    recordz(db, point(e, 3, 6)).

task(CenterX, CenterY, Radius1, Radius2, ResultList):-
    db_filling,
    find_smaller_bigger(Radius1, Radius2, SmallerRadius, BiggerRadius),
    findall(UnknownPointId, inside(UnknownPointId, CenterX, CenterY, SmallerRadius, BiggerRadius), ResultList).

find_smaller_bigger(Radius1, Radius2, SmallerRadius, BiggerRadius) :-
    (   Radius1 =< Radius2
    ->  SmallerRadius = Radius1, BiggerRadius = Radius2
    ;   SmallerRadius = Radius2, BiggerRadius = Radius1
    ).

inside(PointId, CenterX, CenterY, SmallerRadius, BiggerRadius):-
    recorded(db, point(PointId, X, Y)),
    FirstTerm is (X - CenterX) ** 2,
    SecondTerm is (Y - CenterY) ** 2,
    Distance is sqrt(FirstTerm + SecondTerm),
    Distance >= SmallerRadius,
    Distance =< BiggerRadius.