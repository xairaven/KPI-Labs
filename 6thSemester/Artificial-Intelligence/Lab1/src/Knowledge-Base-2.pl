% 19
% У чвертьфіналі Ліги чемпіонів можуть зустрітися 
% 2 футбольних команди з різних підгруп, якщо вони 
% зайняли 1-е або 2-е місце в своїй підгрупі.

meet_in_quarterfinal(Team1, Team2) :-
    winner_round_32(Team1, Group1),
    winner_round_32(Team2, Group2),
    Team1 \= Team2,
    Group1 \= Group2.


winner_round_32(dinamo_zagreb, group1).
winner_round_32(tottenham, group1).
winner_round_32(villareal, group2).
winner_round_32(dynamo_kyiv, group2).
winner_round_32(arsenal, group3).
winner_round_32(olympiacos, group3).
winner_round_32(rangers, group4).
winner_round_32(slavia_prague, group4).
winner_round_32(molde, group5).
winner_round_32(granada, group5).
winner_round_32(ac_milan, group6).
winner_round_32(man_united, group6).
winner_round_32(young_boys, group7).
winner_round_32(ajax, group7).
winner_round_32(shakhtar, group8).
winner_round_32(as_roma, group8).