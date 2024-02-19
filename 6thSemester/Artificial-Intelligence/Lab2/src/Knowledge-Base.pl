% Зовиця (сестра дружини).

% ?Who ?Whom
is_sister_in_law(X, Y) :-
    marriage(_, Y),
    is_female(Y),
    is_sister(X, Y).

% ?Who ?Whom
is_sister(X, Y) :-
    is_parent(F, X),
    is_parent(F, Y),
    is_female(X),
    X \= Y.

% ?Husband, ?Wife
marriage(mike, mary).
marriage(adam, ann).
% ?Parent, ?Child
is_parent(adam, dick).
is_parent(ann, dick).
is_parent(vlad, vika).
is_parent(mike, bob).
is_parent(nastya, vika).
is_parent(mike, kate).
is_parent(mary, kate).
is_parent(vlad, mary).
is_parent(nastya, mary).
is_female(mary).
is_female(ann).
is_female(kate).
is_female(vika).
is_female(nastya).