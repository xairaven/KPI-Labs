USE DBTheater;
GO;

-- Create a view from 1 table --
CREATE VIEW vwActors AS
SELECT last_name, first_name
FROM Actors;
GO;

SELECT * FROM vwActors;

DROP VIEW IF EXISTS vwActors;
GO;

-- Create views from several tables --
CREATE VIEW vwShowsRoles AS
SELECT Shows.title AS [Show Title], Roles.title AS [Role]
FROM Shows
INNER JOIN dbo.Roles on Shows.id = Roles.show_id;
GO;

SELECT * FROM vwShowsRoles;

DROP VIEW IF EXISTS vwShowsRoles;
GO;


-- Try to update view and to create view that updates original table --
CREATE VIEW vwShowsRoles AS
SELECT Shows.title AS [Show Title], Roles.title AS [Role]
FROM Shows
INNER JOIN dbo.Roles on Shows.id = Roles.show_id
WITH CHECK OPTION;
GO;

SELECT * FROM vwShowsRoles;

BEGIN TRANSACTION Operation

UPDATE vwShowsRoles
SET Role='SMTH'
WHERE [Show Title] LIKE 'S%'

SELECT * FROM vwShowsRoles;
SELECT * FROM Roles

ROLLBACK TRANSACTION Operation;

SELECT * FROM vwShowsRoles;

DROP VIEW IF EXISTS vwShowsRoles;
GO;

-- Updating table --
CREATE VIEW vwShowsRoles AS
SELECT Shows.title AS [Show Title], Roles.title AS [Role]
FROM Shows
INNER JOIN dbo.Roles on Shows.id = Roles.show_id
WITH CHECK OPTION;
GO;

SELECT * FROM vwShowsRoles;

BEGIN TRANSACTION Operation

UPDATE Roles
SET title='SMTH'
WHERE show_id > 10

SELECT * FROM vwShowsRoles;
SELECT * FROM Roles

ROLLBACK TRANSACTION Operation;

SELECT * FROM vwShowsRoles;

DROP VIEW IF EXISTS vwShowsRoles;
GO;

-- Create cursor --
CREATE PROCEDURE dbo.spTotalRevenueCursor
    @year INT
AS
BEGIN
    DECLARE foCursor CURSOR FOR
    SELECT budget FROM Shows
    WHERE YEAR(date) = @year;

    DECLARE @SUM INT, @AMOUNT INT;
    SET @SUM = 0;

    OPEN foCursor;
    FETCH NEXT FROM foCursor INTO @AMOUNT;

    WHILE @@FETCH_STATUS=0
    BEGIN
        SET @SUM = @SUM + @AMOUNT;

        FETCH NEXT FROM foCursor INTO @AMOUNT;
    END

    CLOSE foCursor;
    DEALLOCATE foCursor;

    RETURN @SUM;
END
GO;

DECLARE @SUM INT;
EXECUTE @SUM = spTotalRevenueCursor 2025;
SELECT @SUM;

DROP PROCEDURE IF EXISTS dbo.spTotalRevenueCursor;