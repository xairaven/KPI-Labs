/* 4
Чотири білих шашки і чотири чорних розташовані так, як показано на
рисунку. 

[-1;-1;-1;-1;0;1;1;1;1]

Потрібно переставити чорні шашки на клітинки з номерами 6,7,8,9
з дотриманням умов:
1) кожна шашка може перескочити на найближчу клітинку або через одну
клітинку, але не далі;
2) жодна шашка не повинна повертатися на клітинку, де вона вже побувала;
3) в кожній клітинці не повинно бути більше однієї шашки;
4) починати з білої шашки.
*/

% Checker - шашка

% Indexes - checkers, elements - coords of checkers 
initial_state([1, 2, 3, 4, 6, 7, 8, 9]).
% Final state - all black checkers must be on the right side, white - on the left
final_state(List) :-
    nth1(1, List, Checker1), Checker1 >= 6,
    nth1(2, List, Checker2), Checker2 >= 6,
    nth1(3, List, Checker3), Checker3 >= 6,
    nth1(4, List, Checker4), Checker4 >= 6,
    nth1(5, List, Checker5), Checker5 =< 4,
    nth1(6, List, Checker6), Checker6 =< 4,
    nth1(7, List, Checker7), Checker7 =< 4,
    nth1(8, List, Checker8), Checker8 =< 4.

main(Moves):-
    % Get list of coords
    initial_state(List),
    % List of lists - every sublist is history for checker
    History = [[1],[2],[3],[4],[6],[7],[8],[9]],
    dfs(List, History, Moves, FinalResult),
    print_result(Moves, 1),
    print(FinalResult).

% Recursion exit condition
dfs(List, _, [], FinalResult):-
    final_state(List),
    FinalResult = List.
dfs(List, History, [Move | Moves], FinalResult) :-
    % Get checker number. Starting from 5 - first white checker. Can be in any order
    member(Checker, [5, 1, 2, 3, 4, 6, 7, 8]),
    % All variants of delta
    member(Delta, [-2, -1, 1, 2]),
    % Get element of list, where indexes - checkers, elements - coords of checkers 
    nth1(Checker, List, CheckerCord),
    % New coordinate for checker
    NewCord is CheckerCord + Delta,
    % Is new coord on the table?
    legal_bounds(NewCord),
    % Are checkers intersected?
    legal_intersections(List, NewCord),
    % We have to check is current configuration repeated or not
    nth1(Checker, History, CheckerHistoryList),
    \+ member(NewCord, CheckerHistoryList),
    % If all ok, moving checker to the new cell
    move(List, Checker, NewCord, NewList, Move),
    % Append history
    NewCheckerHistoryList = [ NewCord | CheckerHistoryList],
    replace_element_in_list(History, Checker, NewCheckerHistoryList, NewGeneralHistory),
    % If success, heading straight to the next step (by recursion)
    dfs(NewList, NewGeneralHistory, Moves, FinalResult).

% Just changing coord of some checker, and returnung Move string
move(List, Checker, NewCord, ResultList, Move) :-
    Names = [black, black, black, black, white, white, white, white],
    nth1(Checker, List, OldCheckerCord),
    nth1(Checker, Names, CheckerColor),

    replace_element_in_list(List, Checker, NewCord, ResultList),
    Move = CheckerColor-Checker-from-OldCheckerCord-to-NewCord.

% Validation - is coordinate intersect another checker
legal_intersections(List, NewCord) :-
    \+ member(NewCord, List).
    
% Is new coord on the table?
legal_bounds(NewCord) :-
    NewCord >= 1, NewCord =< 9.

% Auxiliary predicates 
% Predicate for replacing one element to another by index
replace_element_in_list(List, Index, NewElem, Result) :-
    nth1(Index, List, _, ListWithoutElement),
    nth1(Index, Result, NewElem, ListWithoutElement).

print_result([], _).
print_result([Head | Tail], Number) :-
    format("~d) ~w", [Number, Head]),
    nl,
    NewNumber is Number + 1,
    print_result(Tail, NewNumber). 