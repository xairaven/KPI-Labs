% Left Recursion

% 19
% x^n, x < n
% n, x > n

% +X, +N, -Result
func(X, N, Result) :- X < N, pow(X, N, Result).
func(X, N, Result) :- X is N, Result is nan.
func(X, N, N) :- X > N.

% +X, +N, -Result
pow(X, N, Result) :- N > 0, pow_positive(X, N, Result).
pow(_, 0, 1).
pow(X, N, Result) :- N < 0, pow_negative(X, N, Result).

% +X, +N, +Accumulator, -Result
pow_positive(_, 0, 1).
pow_positive(X, N, Result) :-
    N > 0,
    N_Temp is N - 1,
    pow_positive(X, N_Temp, Result_Temp),
    Result is Result_Temp * X.

% +X, +N, -Result
pow_negative(X, N, Result) :- 
    Positive_N is N * -1,
    pow(X, Positive_N, Denominator),
    Result is 1 / Denominator.