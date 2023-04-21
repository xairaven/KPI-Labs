USE theateractors;

# Max years of experience
SELECT MAX(years_of_experience)
FROM actors;

# Comparing salaries to corresponding max bonuses
SELECT salary, MAX(bonus) FROM actors
INNER JOIN contracts c on actors.id = c.Actors_id
GROUP BY salary;

# Min years of experience
SELECT MIN(years_of_experience)
FROM actors;

# Comparing salaries to corresponding min bonuses
SELECT salary, MIN(bonus) FROM actors
INNER JOIN contracts c on actors.id = c.Actors_id
GROUP BY salary;

# Average salary in theater
SELECT AVG(salary)
FROM contracts;

# Average bonuses comparing to salaries
SELECT salary, AVG(bonus) AS average FROM actors
INNER JOIN contracts c on actors.id = c.Actors_id
GROUP BY salary
ORDER BY average DESC, salary DESC;

# Amount of actors that played at least 1 role
SELECT COUNT(DISTINCT Actors_id) AS Actors
FROM additionaltable;

# Shows by months
SELECT COUNT(title) AS 'shows', MONTH(date) AS month
FROM shows
GROUP BY month
ORDER BY month;

# Spread of salaries across ranges
SELECT salary, COUNT(Actors_id)
FROM contracts
GROUP BY salary
ORDER BY salary DESC;

# Error
/*
SELECT salary, Actors_id, COUNT(Actors_id)
FROM contracts
GROUP BY salary
ORDER BY salary DESC;
*/

# Find avg of performed shows by years of experience
SELECT years_of_experience AS years,
       COUNT(last_name) AS people,
       AVG(spectacle_counter) as average
FROM actors
GROUP BY years
HAVING average > 0
ORDER BY years DESC;

# Error: Where with aggregate function
/*
SELECT last_name, years_of_experience
FROM actors
WHERE years_of_experience < AVG(years_of_experience);
 */

# WHERE with sub-query
SELECT last_name, years_of_experience
FROM actors
WHERE years_of_experience < (SELECT AVG(years_of_experience) FROM actors)
ORDER BY years_of_experience DESC;

# ORDER BY example
SELECT date, budget
FROM shows
ORDER BY date;

# GROUP BY example
SELECT YEAR(date), SUM(date)
FROM shows
GROUP BY YEAR(date);

# DISTINCT example
SELECT DISTINCT Actors_id
FROM additionaltable
ORDER BY 1;