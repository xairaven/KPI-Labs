USE DBTheater
GO;

-- Actors, their salary, and average show budget
SELECT first_name, last_name, salary, AVG(budget) AS [Average show budget]
FROM Actors
INNER JOIN dbo.Contracts C on Actors.id = C.actor_id
INNER JOIN dbo.Cast C2 on Actors.id = C2.actor_id
INNER JOIN dbo.Roles R2 on C2.role_id = R2.id
INNER JOIN dbo.Shows S on R2.show_id = S.id
GROUP BY first_name, last_name, salary
GO;

-- Checking the results of the first script (have to match)
SELECT * FROM Actors
LEFT JOIN (SELECT A.id, AVG(budget) AS [Average show budget]
FROM Shows
INNER JOIN dbo.Roles R2 on Shows.id = R2.show_id
INNER JOIN dbo.Cast C on R2.id = C.role_id
INNER JOIN dbo.Actors A on A.id = C.actor_id
INNER JOIN dbo.Contracts C2 on A.id = C2.actor_id
GROUP BY a.id) S ON S.id = Actors.id
GO;