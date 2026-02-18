/*
Одного разу зібралися сім пар. Прізвища чоловіків: Владимиров, Федоров,
Вікторов, Степанов, Баранов, Харламов і Тарасов. Жінок звуть: Тоня, Олена,
Люся, Світлана, Маша, Оля і Галя.
Владимиров танцював з Оленою і Світланою, Харламов - з Машею і
Світланою, Тарасов - з Оленою та Олею, Вікторов - з Оленою, Степанов - зі
Світланою, а Баранов - з Олею.
Потім грали в карти. Спочатку Вікторів і Владимиров грали з Олею і
Галею, потім чоловіків змінили Степанов і Харламов, а жінки продовжували
грати. І, нарешті, Степанов і Харламов зіграли одну партію з Тонею і
Оленою.
На вечорі жоден чоловік не танцював зі своєю дружиною і ні одна
подружня пара не сідала одночасно грати в карти.
Хто на кому одружений?
*/

% vladimirov, fedorov, viktorov, stepanov, baranov, harlamov, tarasov
% tonya, olena, lyusya, svitlana, masha, olya, galya

male(vladimirov).
male(fedorov).
male(viktorov).
male(stepanov).
male(baranov).
male(harlamov).
male(tarasov).

female(tonya).
female(olena).
female(lyusya).
female(svitlana).
female(masha).
female(olya).
female(galya).

dance(vladimirov, olena).
dance(vladimirov, svitlana).
dance(harlamov, masha).
dance(harlamov, svitlana).
dance(tarasov, olena).
dance(tarasov, olya).
dance(viktorov, olena).
dance(stepanov, svitlana).
dance(baranov, olya).


puzzle(MarriageList) :-
    can_be_in_marriage(vladimirov, Female1),
    can_be_in_marriage(fedorov, Female2),
    can_be_in_marriage(viktorov, Female3),
    can_be_in_marriage(stepanov, Female4),
    can_be_in_marriage(baranov, Female5),
    can_be_in_marriage(harlamov, Female6),
    can_be_in_marriage(tarasov, Female7),
    unique([Female1, Female2, Female3, Female4, Female5, Female6, Female7]),

    MarriageList = [
        marriage(vladimirov, Female1),
        marriage(fedorov, Female2),
        marriage(viktorov, Female3),
        marriage(stepanov, Female4),
        marriage(baranov, Female5),
        marriage(harlamov, Female6),
        marriage(tarasov, Female7)
    ].

can_be_in_marriage(X, Y) :-
    Cards1 = [viktorov, vladimirov, olya, galya],
    Cards2 = [stepanov, harlamov, olya, galya],
    Cards3 = [stepanov, harlamov, tonya, olena],
    male(X),
    female(Y),
    not(dance(X, Y)),
    cards_pair(X, Y, Cards1),
    cards_pair(X, Y, Cards2),
    cards_pair(X, Y, Cards3).

cards_pair(X, Y, List) :-
    not(member(X, List)), member(Y, List).
cards_pair(X, Y, List) :-
    member(X, List), not(member(Y, List)).
cards_pair(X, Y, List) :-
    not(member(X, List)), not(member(Y, List)).

unique([]).
unique([X|Xs]) :- \+ member(X, Xs), unique(Xs).