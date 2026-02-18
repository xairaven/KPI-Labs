USE theateractors;

INSERT INTO actors (first_name, last_name, hiring_date)
    SELECT first_name, last_name, (CURRENT_DATE - INTERVAL FLOOR(RAND() * 3650) DAY)
    FROM sakila.actor
    LIMIT 20;

INSERT INTO shows (title, budget, date)
    SELECT DISTINCT title, (ROUND(RAND() * 400000)), (CURRENT_DATE + INTERVAL FLOOR(RAND() * 365) DAY)
    FROM sakila.film
    ORDER BY rand()
    LIMIT 15;

INSERT INTO contracts (base_salary, Actors_id)
    SELECT ROUND(RAND() * 10000, -3), id
    FROM actors
    WHERE actors.id BETWEEN 6 AND 25;

INSERT INTO roles (title)
    SELECT last_name
    FROM sakila.customer
    ORDER BY rand()
    LIMIT 15;