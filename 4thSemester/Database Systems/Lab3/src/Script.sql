USE theateractors;

# Surnames of actors:
SELECT last_name
FROM actors;

# Trainees without salary:
SELECT Actors_id
FROM contracts
WHERE base_salary = 0 AND bonus = 0;

# Shows, that will not take place in 2023
SELECT title, date
FROM shows
WHERE NOT YEAR(date) = 2023;

# Actors, that have more than 5 years of experience OR more than 2 played spectacles
SELECT first_name, last_name, years_of_experience, spectacle_counter
FROM actors
WHERE spectacle_counter > 2 OR years_of_experience > 5;

# Actors hired in 2013 and 2014
SELECT first_name, last_name, hiring_date
FROM actors
WHERE YEAR(hiring_date) IN (2013, 2014);

# Actors hired between 2012 and 2015
SELECT first_name, last_name, hiring_date
FROM actors
WHERE YEAR(hiring_date) BETWEEN 2012 AND 2015;

# Actors whose last name starts with the letter D
SELECT first_name, last_name
FROM actors
WHERE last_name LIKE 'D%';

# Actors whose reward field is not NULL:
SELECT first_name, last_name, awards
FROM actors
WHERE awards IS NOT NULL;

# Actors sorted by years of experience (DESC) and by last name
SELECT last_name, first_name, years_of_experience
FROM actors
ORDER BY years_of_experience DESC, last_name;

# Inner join (using syntax) - Contracts and Actors
SELECT last_name, first_name, salary
FROM contracts
INNER JOIN actors USING (id);

# All roles and actors who play them:
SELECT actors.last_name, actors.first_name, r.title
FROM actors
LEFT JOIN additionaltable AS a ON actors.id = a.Actors_id
RIGHT JOIN roles r on a.Roles_id = r.id;

# All actors and their roles:
SELECT actors.last_name, actors.first_name, r.title
FROM actors
LEFT JOIN additionaltable AS a ON actors.id = a.Actors_id
LEFT JOIN roles r on a.Roles_id = r.id;

# Actors that played at least 1 role and their roles:
SELECT actors.last_name, actors.first_name, r.title
FROM actors
LEFT JOIN additionaltable AS a ON actors.id = a.Actors_id
INNER JOIN roles r on a.Roles_id = r.id;

# Possible applicants for the role among the actors who are free
SELECT DISTINCT actors.last_name, actors.first_name, r.title
FROM actors
LEFT JOIN additionaltable AS a ON actors.id = a.Actors_id
CROSS JOIN roles AS r
WHERE a.Roles_id IS NULL;

# Full outer join on Actors and Roles
SELECT actors.last_name, actors.first_name, r.title
FROM actors
LEFT JOIN additionaltable a ON actors.id = a.Actors_id
LEFT JOIN roles r ON a.Roles_id = r.id
UNION ALL
SELECT actors.last_name, actors.first_name, r.title
FROM roles AS r
LEFT JOIN additionaltable a on r.id = a.Roles_id
LEFT JOIN actors on a.Actors_id = actors.id
WHERE actors.last_name IS NULL;

# Left inner join (Actors without roles)
SELECT last_name, first_name
FROM Actors
LEFT JOIN AdditionalTable ON Actors.id = AdditionalTable.Actors_id
WHERE Roles_id IS NULL;

# Left inner join with EXISTS operator (Actors without roles)
SELECT DISTINCT last_name, first_name
FROM actors AS act1
WHERE NOT EXISTS(SELECT *
FROM actors AS act2
INNER JOIN additionaltable a ON act2.id = a.Actors_id
INNER JOIN roles ON Roles_id = roles.id
WHERE act1.id = act2.id);

# 3rd-5th actor ordered by salary
SELECT last_name, first_name, salary
FROM actors
JOIN contracts c ON actors.id = c.Actors_id
ORDER BY salary DESC, last_name
LIMIT 3
OFFSET 2;