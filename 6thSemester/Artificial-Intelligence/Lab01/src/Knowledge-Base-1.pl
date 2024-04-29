% 19
% У чвертьфіналі Ліги чемпіонів можуть зустрітися 
% 2 футбольних команди з різних підгруп, якщо вони 
% зайняли 1-е або 2-е місце в своїй підгрупі.

meet_in_quarterfinal(Team1, Team2) :-
    winner_round_32(Team1),
    winner_round_32(Team2),
    in_group(Team1, Group1),
    in_group(Team2, Group2),
    Group1 \= Group2. 

winner_round_32(dinamo_zagreb).
winner_round_32(tottenham).
winner_round_32(villarreal).
winner_round_32(dynamo_kyiv).
winner_round_32(arsenal).
winner_round_32(olympiacos).
winner_round_32(rangers).
winner_round_32(slavia_prague).
winner_round_32(molde).
winner_round_32(granada).
winner_round_32(ac_milan).
winner_round_32(man_united).
winner_round_32(young_boys).
winner_round_32(ajax).
winner_round_32(shakhtar).
winner_round_32(as_roma).

in_group(dinamo_zagreb, group1).
in_group(krasnodar, group1).
in_group(tottenham, group1).
in_group(wolfsberg, group1).

in_group(villarreal, group2).
in_group(rb_salzburg, group2).
in_group(club_brugge, group2).
in_group(dynamo_kyiv, group2).

in_group(arsenal, group3).
in_group(benfica, group3).
in_group(psv, group3).
in_group(olympiacos, group3).

in_group(rangers, group4).
in_group(antwerp, group4).
in_group(leicester_city, group4).
in_group(slavia_prague, group4).

in_group(hoffenheim, group5).
in_group(molde, group5).
in_group(napoli, group5).
in_group(granada, group5).

in_group(ac_milan, group6).
in_group(red_star_belgrade, group6).
in_group(man_united, group6).
in_group(real_sociedad, group6).

in_group(leverkjsen, group7).
in_group(young_boys, group7).
in_group(ajax, group7).
in_group(lillf_osc, group7).

in_group(shakhtar, group8).
in_group(maccabi_tel_aviv, group8).
in_group(as_roma, group8).
in_group(braga, group8).