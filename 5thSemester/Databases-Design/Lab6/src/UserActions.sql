USE DBTheater;
GO;

-- EXECUTE AS USER = 'AlexUser';

-- Our Table
SELECT * FROM Actors ORDER BY id OFFSET 499 ROWS;

-- User perms, username, server username
SELECT * FROM sys.fn_my_permissions('Actors', 'object');
SELECT CURRENT_USER;
SELECT suser_name();

-- Testing inserting
INSERT INTO Actors(first_name, last_name, total_experience)
VALUES ('Alex', 'Kovalov', 99);

-- Deleting test results
DELETE FROM Actors WHERE id=502;