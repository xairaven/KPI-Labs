% 19
% Swap the n-th and (n+1)-th elements of the list.
% The number n is specified in the request.

% +InitialList, 1, -ResultList
swap([A, B | Tail], 1, [B, A | Tail]).

% +InitialList, +N, -ResultList
swap([Head | Tail], N, [Head | ResultTail]) :-
    N > 1,
    N_Next is N - 1,
    swap(Tail, N_Next, ResultTail).