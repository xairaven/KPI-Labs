% 19
% Task: get a list of values of the function specified in the individual task.
% f(x) = x^N * sum_(i=1)^(N) (i/(x+i))

% +X, +X-Final, +H,
% +I, +N, -ResultList
function(X, X_Final, H, I, N, ResultList) :-
    function_list(X, X_Final, H, I, N, ResultList).

% +X, +X-Final, +H,
% +I, +N, -ResultList
function_list(X_Current, X_Final, H, I, N, [Head|Tail]) :-
    X_Current =< X_Final,
    pow(X_Current, N, FirstFactor),
    sum(X_Current, I, N, SecondFactor),
    Head is FirstFactor * SecondFactor,
    X_Next = X_Current + H,
    function_list(X_Next, X_Final, H, I, N, Tail).
function_list(X_Current, X_Final, _, _, _, []) :-
    X_Current > X_Final.

% +X, +I, +N, -Result
sum(X, I, N, Result) :- sum_accum(X, I, N, 0, Result).
sum_accum(X, I, N, Accumulator, Result) :-
    I =< N,
    Accumulator_Temp is Accumulator + (I / (X + I)),
    I_Next is I + 1,
    sum_accum(X, I_Next, N, Accumulator_Temp, Result).
sum_accum(_, I, N, Result, Result) :- I > N.

% +X, +N, -Result
pow(X, N, Result) :- N > 0, pow_positive(X, N, 1, Result).
pow(_, 0, 1).
pow(X, N, Result) :- N < 0, pow_negative(X, N, Result).

% +X, +N, +Accumulator, -Result
pow_positive(_, 0, Result, Result).
pow_positive(X, N, Accumulator, Result) :-
    N > 0,
    N_Temp is N - 1,
    Accumulator_Temp is Accumulator * X,
    pow_positive(X, N_Temp, Accumulator_Temp, Result).

% +X, +N, -Result
pow_negative(X, N, Result) :- 
    Positive_N is N * -1,
    pow(X, Positive_N, Denominator),
    Result is 1 / Denominator.