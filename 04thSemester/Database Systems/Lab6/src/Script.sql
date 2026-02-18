USE TheaterActors;

# Union operation. Actors name from TheaterActors and Sakila databases
SELECT last_name, first_name
FROM TheaterActors.actors
UNION
SELECT last_name, first_name
FROM sakila.actor
ORDER BY last_name;

# Intersect operation. Actors name from TheaterActors and Sakila databases
SELECT last_name, first_name
FROM TheaterActors.actors AS MainActors
INNER JOIN sakila.actor AS SakilaActors
USING (last_name, first_name);

# Except operation. Actors name from TheaterActors and Sakila databases
SELECT last_name, first_name
FROM TheaterActors.actors AS A
WHERE NOT EXISTS (SELECT last_name, first_name
FROM sakila.actor AS B
INNER JOIN theateractors.actors
USING (last_name, first_name)
WHERE A.last_name = B.last_name AND A.first_name = B.first_name);

# Cartesian product operation. Possible applicants for the role among the actors who are free
SELECT DISTINCT actors.last_name, actors.first_name, r.title
FROM actors
LEFT JOIN additionaltable AS a ON actors.id = a.Actors_id
CROSS JOIN roles AS r
WHERE a.Roles_id IS NULL;

# Selection operation. Actors with 'D' as first letter of surname
SELECT last_name, first_name, awards
FROM actors
WHERE last_name LIKE 'D%'
ORDER BY last_name;

# Projection operation. Title and date of shows
SELECT DISTINCT title, date
FROM shows;

# Join operation. Actors and their roles
SELECT actors.last_name, actors.first_name, r.title
FROM actors
INNER JOIN additionaltable AS a ON actors.id = a.Actors_id
INNER JOIN roles r on a.Roles_id = r.id;

# Division operation. Get actors from list of possible roles for every unemployed actor
SELECT PossibleRoles.id, PossibleRoles.last_name, PossibleRoles.first_name
FROM (SELECT DISTINCT actors.id, actors.last_name, actors.first_name, r.title
FROM actors
LEFT JOIN additionaltable AS a ON actors.id = a.Actors_id
CROSS JOIN roles AS r
WHERE a.Roles_id IS NULL) AS PossibleRoles
GROUP BY PossibleRoles.id, PossibleRoles.last_name, PossibleRoles.first_name
HAVING COUNT(PossibleRoles.title) = COUNT(DISTINCT PossibleRoles.title);

# Extend operation. Count years from hiring
SELECT last_name, first_name, hiring_date,
       ROUND(DATEDIFF(CURDATE(), hiring_date) / 365) AS YearsFromHiring
FROM actors;

# Group by operation. Count shows by month
SELECT MONTH(date) AS month, COUNT(title) AS shows
FROM shows
GROUP BY month
ORDER BY month;

# Rename operation. Employed actors = friends
SELECT DISTINCT first_name AS friends
FROM actors
INNER JOIN additionaltable a on actors.id = a.Actors_id
INNER JOIN roles r on a.Roles_id = r.id;