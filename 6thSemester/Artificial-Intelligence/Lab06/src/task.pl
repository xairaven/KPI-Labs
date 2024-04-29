% 19
% Given a list whose length is a multiple of three. 
% Get a list in which the last third is kept 
% and the first and second are swapped. 
% Moreover, the elements of the second third will be in the reverse order.

% Main Predicate
% +List, -Result
process_list(List, Result) :-
    length(List, Length),
    ThirdLength is Length // 3,
    split(List, ThirdLength, FirstPart, Rest),
    split(Rest, ThirdLength, SecondPart, ThirdPart),
    reverse_list(FirstPart, ReversedFirstPart),
    append(SecondPart, ReversedFirstPart, Temp),
    append(Temp, ThirdPart, Result).

% Predicate for splitting list on two parts - first by N element and second (rest).
% +InputList, +LengthOfFirstPart, -First, -Rest
split(List, N, First, Rest) :-
    length(First, N),
    append(First, Rest, List).

% Predicate for reversing the list
% +InputList, -ReversedList
reverse_list([], []).
reverse_list([X|Rest], Reversed) :-
    reverse_list(Rest, ReversedRest),
    append(ReversedRest, [X], Reversed).