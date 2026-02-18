USE theateractors;

# Job position corresponding to years of experience. Simple CASE example.
SELECT last_name, first_name, YEAR(hiring_date) AS 'Hiring Year',
    CASE
        WHEN YEAR(hiring_date) < 2016 THEN 'Senior'
        WHEN YEAR(hiring_date) BETWEEN 2016 AND 2020 THEN 'Middle'
        ELSE 'Junior'
    END AS Position
FROM actors;

# In what year will the spectacle be performed? Logical CASE example.
SELECT title,
       CASE YEAR(date)
           WHEN YEAR(CURDATE()) THEN 'This year!'
           WHEN YEAR(CURDATE()) + 1 THEN 'Next year!'
           ELSE 'Spectacle already played!'
       END year
FROM shows;

# Salary coolness. IF function example.
SELECT last_name, first_name, salary,
       IF(salary >= (SELECT AVG(salary) from contracts),
           'Wow, that is cool salary!', 'Sufferable.') AS 'Salary coolness'
FROM contracts
INNER JOIN actors AS a ON contracts.Actors_id = a.id;

# Did the actor play at least one performance? SELECT IF example.
SELECT last_name, first_name,
       IF(spectacle_counter > 0, '+', '-') AS 'Played spectacles'
FROM actors;

# Choose all actors that aren't Trainees. IF with WHERE and IN operators example.
SELECT last_name, first_name, salary
FROM actors
INNER JOIN contracts c on actors.id = c.Actors_id
WHERE
    CASE
        WHEN salary >= (SELECT AVG(salary) from contracts)
            THEN 'Higher than average'
        WHEN salary = 0
            THEN 'Trainee'
        ELSE 'Lower than average'
    END NOT IN('Trainee');

# Order by spectacles, then on last name. IF with ORDER BY operator example.
SELECT last_name, first_name, spectacle_counter
FROM actors
ORDER BY IF(spectacle_counter = 0, last_name, spectacle_counter);

# Look over the range of salaries and the number of actors.
# If the salary is higher than the average - show them all,
# otherwise - only if there are more than two such actors
SELECT salary, COUNT(Actors_id) AS employees
FROM contracts
GROUP BY salary
HAVING employees >
       CASE
              WHEN salary > (SELECT AVG(salary) FROM contracts) THEN 0
              ELSE 2
       END
ORDER BY salary DESC;

# Set value for column IsEmployeeSenior. IF with UPDATE example.
UPDATE actors
SET IsEmployeeSenior =
        IF(years_of_experience > 5, '+', '-')
WHERE years_of_experience <> 0;

# Delete actors with 0 years of experience, if they don't have awards
# IF with DELETE example.
DELETE FROM actors
WHERE years_of_experience = IF(awards = '', 0, -1);

# Select all actors who are not seniors. CHOOSE example.
SELECT last_name, IsEmployeeSenior
FROM actors
WHERE IsEmployeeSenior = ELT(2, '+', '-');

# If actor is trainee, print it. COALESCE example.
SELECT last_name, first_name,
       COALESCE(IsEmployeeSenior, 'Trainee') AS 'Is Senior'
FROM actors
ORDER BY last_name;

# Same as previous, but IFNULL example.
SELECT last_name, first_name,
       IFNULL(IsEmployeeSenior, 'Trainee') AS 'Is Senior'
FROM actors
ORDER BY last_name;

# FULL OUTER JOIN on Roles and Actors
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