-- Queries --
USE DBTheater;
GO;

-- Selection of individual fields.
SELECT TOP 10 first_name, last_name
FROM dbo.Actors;
GO;

-- Select all columns.
SELECT TOP 10 *
FROM dbo.Shows;
GO;

-- Sort by several fields.
SELECT TOP 10 *
FROM dbo.Shows
ORDER BY budget DESC, title
GO;

-- Query with GROUP BY and HAVING statements
---- Shows with budget > 5000 and with 2+ actors
SELECT S.id, S.title, SUM(S.budget) AS total_budget, COUNT(C.actor_id) AS actors_count
FROM Shows S
LEFT JOIN Roles R ON S.id = R.show_id
LEFT JOIN Cast C ON R.id = C.role_id
GROUP BY S.id, S.title
HAVING SUM(S.budget) > 5000 AND COUNT(C.actor_id) > 3;
GO;

-- Query with GROUP BY and ORDER BY statements
---- Group "Total experience" by awards number. Sort by experience
SELECT total_experience AS [Total Experience], COUNT(awards) [Awards Numbers]
FROM Actors
GROUP BY total_experience
ORDER BY total_experience DESC
GO;

-- Inner Join
---- Contracts and corresponding Actors
SELECT last_name, first_name, salary
FROM Contracts
INNER JOIN Actors ON Contracts.actor_id = Actors.id
GO;

-- Left & Right Join
---- All roles and actors who play them:
SELECT last_name, first_name, title
FROM Actors
LEFT JOIN Cast c ON actors.id = c.actor_id
RIGHT JOIN Roles r on c.role_id = r.id;
GO;

-- Filtering
---- AND: Select junior actors (Salary <= 10000, 3 years of experience):
SELECT first_name, last_name, salary, hiring_date
FROM Contracts
JOIN dbo.Actors A on A.id = Contracts.actor_id
WHERE salary <= 10000 AND YEAR(GETDATE()) - YEAR(hiring_date) <= 3
GO;

---- NOT: Shows, that will not take place in current year
SELECT title, date
FROM Shows
WHERE NOT YEAR(date) = YEAR(GETDATE());
GO;

---- OR: Actors, that have more than 3 years of experience OR salary more than 10000
SELECT first_name, last_name, salary, hiring_date
FROM Contracts
JOIN dbo.Actors A on A.id = Contracts.actor_id
WHERE salary >= 10000 OR YEAR(GETDATE()) - YEAR(hiring_date) >= 3
GO;

---- BETWEEN: Actors hired between 2012 and 2015
SELECT first_name, last_name, YEAR(hiring_date) AS [Hiring Year]
FROM Actors
INNER JOIN dbo.Contracts C on Actors.id = C.actor_id
WHERE YEAR(hiring_date) BETWEEN 2012 AND 2015
GO;

---- IN: Actors hired in 2013 and 2014
SELECT first_name, last_name, hiring_date
FROM Actors
INNER JOIN dbo.Contracts C on Actors.id = C.actor_id
WHERE YEAR(hiring_date) IN (2013, 2014)
GO;

---- NOT IN: Actors hired not in 2013 and 2014
SELECT first_name, last_name, hiring_date
FROM Actors
INNER JOIN dbo.Contracts C on Actors.id = C.actor_id
WHERE YEAR(hiring_date) NOT IN (2013, 2014)
GO;

---- LIKE: Actors whose last name starts with the letter D
SELECT first_name, last_name
FROM Actors
WHERE last_name LIKE 'D%'
GO;

---- NOT LIKE: Actors whose last name not starts with the letter D
SELECT first_name, last_name
FROM Actors
WHERE last_name NOT LIKE 'D%'
GO;

---- EXISTS: Actors without roles
SELECT DISTINCT last_name, first_name
FROM Actors AS Act1
WHERE NOT EXISTS(SELECT *
FROM Actors AS Act2
INNER JOIN Cast C ON act2.id = C.actor_id
INNER JOIN Roles ON c.role_id = Roles.id
WHERE Act1.id = Act2.id)
GO;

-- NULL: Actors without roles
SELECT last_name, first_name
FROM Actors
LEFT JOIN Cast ON Actors.id = Cast.actor_id
WHERE role_id IS NULL
GO;

-- NOT NULL: Actors whose reward field is not NULL:
SELECT first_name, last_name, awards
FROM Actors
WHERE awards IS NOT NULL
GO;

-- Aggregate functions
---- Max years of experience
SELECT MAX(YEAR(GETDATE()) - YEAR(hiring_date))
FROM Contracts
GO;

---- Min years of experience
SELECT MIN(YEAR(GETDATE()) - YEAR(hiring_date))
FROM Contracts
GO;

---- Average salary in theater
SELECT AVG(salary)
FROM Contracts
GO;

---- Amount of actors that played at least 1 role
SELECT COUNT(DISTINCT actor_id) AS Actors
FROM Cast
GO;

-- Using CASE, IIF, ISNULL
---- Simple Case: Job position corresponding to years of experience.
SELECT last_name, first_name, YEAR(hiring_date) AS 'Hiring Year',
    CASE
        WHEN YEAR(hiring_date) < 2016 THEN 'Senior'
        WHEN YEAR(hiring_date) BETWEEN 2016 AND 2020 THEN 'Middle'
        ELSE 'Junior'
    END AS Position
FROM Actors
INNER JOIN dbo.Contracts C on Actors.id = C.actor_id
GO;

---- Logical Case: In what year will the spectacle be performed?
SELECT title as Title, YEAR(date) AS YEAR,
       CASE YEAR(date)
           WHEN YEAR(GETDATE()) THEN 'This year!'
           WHEN YEAR(GETDATE()) + 1 THEN 'Next year!'
           ELSE 'Spectacle already played or there is still a long wait for it!'
       END [WHEN]
FROM Shows
GO;

----  IIF: Comparing salary to average
SELECT last_name, first_name, salary,
       IIF(salary >= (SELECT AVG(salary) FROM Contracts),
           'Bigger or equal', 'Lower') AS [Comparing to AVG salary]
FROM Contracts
INNER JOIN Actors AS a ON Contracts.actor_id = a.id
GO;

---- ISNULL: Do actor have awards?
SELECT last_name, first_name,
       ISNULL(awards, 'Actor do not have any awards') AS [Awards]
FROM Actors
GO;

-- Relational algebra
---- Union operation. Actors and user names
SELECT last_name, first_name
FROM Actors
UNION
SELECT last_name, first_name
FROM Users
ORDER BY last_name
GO;

---- INTERSECT OPERATION: Users with the same actor username
SELECT last_name, first_name
FROM Actors
INTERSECT
SELECT last_name, first_name
FROM Users
GO;

---- EXCEPT OPERATION: Users and actors with different usernames
SELECT last_name, first_name
FROM Actors
EXCEPT
SELECT last_name, first_name
FROM Users
GO;

---- Join operation. Actors and their roles
SELECT last_name, first_name, title
FROM Actors
INNER JOIN Cast AS C ON actors.id = C.role_id
INNER JOIN roles AS R on C.role_id = R.id
GO;

---- Cartesian product operation.
---- Possible applicants for the role among the actors who are free
SELECT DISTINCT A.last_name, A.first_name, R.title
FROM Actors as A
LEFT JOIN Cast AS C ON A.id = C.actor_id
CROSS JOIN Roles AS R
WHERE C.role_id IS NULL
GO;

---- Selection operation. Actors with 'A' as first letter of surname
SELECT last_name, first_name, awards
FROM Actors
WHERE last_name LIKE 'A%'
ORDER BY last_name
GO;

---- Projection operation. Title and date of shows
SELECT DISTINCT title, date
FROM shows
GO;

---- Division operation. Get actors from list of possible roles for every unemployed actor
SELECT PossibleRoles.id, PossibleRoles.last_name, PossibleRoles.first_name
FROM (SELECT DISTINCT Actors.id, last_name, first_name, title
FROM Actors
LEFT JOIN Cast AS C ON Actors.id = C.actor_id
CROSS JOIN Roles AS R
WHERE C.role_id IS NULL) AS PossibleRoles
GROUP BY PossibleRoles.id, PossibleRoles.last_name, PossibleRoles.first_name
HAVING COUNT(PossibleRoles.title) = COUNT(DISTINCT PossibleRoles.title)
GO;

---- Extend operation. Count years from hiring
SELECT last_name, first_name, hiring_date,
       DATEDIFF(year, hiring_date, GETDATE()) AS [Years from hire]
FROM Actors
INNER JOIN dbo.Contracts C on Actors.id = C.actor_id
GO;

---- Group by operation. Count shows by month
SELECT MONTH(date) AS [Month], COUNT(title) AS [Shows]
FROM Shows
GROUP BY MONTH(date)
ORDER BY month
GO;

---- Rename operation (AS VARIANT).
SELECT DISTINCT actor_id AS ActorId, role_id AS RoleId
FROM Cast
GO;

---- Rename operation (SQUARE BRACKETS VARIANT).
SELECT DISTINCT actor_id [Actor Identifier], role_id [Role Identifier]
FROM Cast
GO;