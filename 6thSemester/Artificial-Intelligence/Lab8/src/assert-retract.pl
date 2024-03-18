% 19
% ASSERT-RETRACT MECHANISM
% Using the given center and radiuses of two concentric circles, 
% find all points belonging to the ring formed by the given circles.

:- dynamic
    point/3.

%?Point ID, ?X, ?Y
db_filling:-
    point(_,_,_), !.

db_filling:-
    assert(point(a, 4, 4)),
    assert(point(b, 2, 5)),
    assert(point(c, 2, 8)),
    assert(point(d, 5, 1)),
    assert(point(e, 3, 6)).

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
    point(PointId, X, Y),
    FirstTerm is (X - CenterX) ** 2,
    SecondTerm is (Y - CenterY) ** 2,
    Distance is sqrt(FirstTerm + SecondTerm),
    Distance >= SmallerRadius,
    Distance =< BiggerRadius.