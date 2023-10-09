USE DBTheater;
GO;

-- Create 1 stored procedure without parameters --
-- Task: Print all actors with awards --
CREATE PROCEDURE dbo.spActorsWithAwards AS
    SELECT last_name, first_name, awards
    FROM Actors
    WHERE awards IS NOT NULL
    ORDER BY last_name;
GO;

EXECUTE dbo.spActorsWithAwards;

-- Location of procedure --
SELECT * FROM INFORMATION_SCHEMA.ROUTINES
WHERE SPECIFIC_NAME LIKE 'spActorsWithAwards';

DROP PROCEDURE IF EXISTS spActorsWithAwards;
GO;


-- Create 1 stored procedures with an input parameter --
-- Task: Get shows by year
CREATE PROCEDURE dbo.spShowByYear
    @Year INT = NULL
AS
BEGIN
    IF @Year IS NULL
        BEGIN
           SET @Year = YEAR(GETDATE())
        END

    SELECT title AS [Title],
           CONCAT(MONTH(date), '-', YEAR(date)) AS [Date],
           budget AS [Budget]
    FROM Shows
    WHERE YEAR(date) = @Year
    ORDER BY [Title]
END
GO;

EXECUTE dbo.spShowByYear @Year = 2025;
GO;

-- Create 2 stored procedure with an output parameter --
-- Task: Get total experience of an actor (total_experience + time from hiring)
CREATE PROCEDURE dbo.spYearsOfExperience
    @actorId INT,
    @years INT OUTPUT
AS
BEGIN
    SELECT @years = (DATEDIFF(year, hiring_date, GETDATE()) + total_experience)
    FROM Actors
    INNER JOIN dbo.Contracts on Actors.id = Contracts.actor_id
    WHERE Actors.id = @actorId
END
GO;

DECLARE @ActorId INT, @years INT;
SELECT @ActorId = id FROM Actors WHERE last_name LIKE 'Aprile' AND first_name LIKE 'Elie';
EXECUTE dbo.spYearsOfExperience @ActorId, @years OUT;
SELECT @years;

DROP PROCEDURE IF EXISTS dbo.spYearsOfExperience;
GO;

-- Task: Get revenue from all shows by year--
CREATE PROCEDURE dbo.spRevenueByYear
    @year INT
AS
BEGIN
    DECLARE @Shows TABLE (
        Title NVARCHAR(50),
        Date NVARCHAR(50),
        Budget MONEY
    );

    DECLARE @Result INT;

    INSERT INTO @Shows (Title, Date, Budget)
    EXEC dbo.spShowByYear @year;

    SELECT @Result = SUM(budget) FROM @Shows;

    RETURN @Result;
END
GO;

DECLARE @TotalRevenue INT;
EXEC @TotalRevenue = dbo.spRevenueByYear 2025;
SELECT 'Total revenue' = @TotalRevenue;

DROP PROCEDURE IF EXISTS dbo.spShowByYear;
DROP PROCEDURE IF EXISTS dbo.spRevenueByYear;
GO;


-- Create 2 user-defined functions --
-- Task: Count all roles without actors working on it --
CREATE FUNCTION dbo.fnAvailableRoles()
    RETURNS INT AS
BEGIN
    DECLARE @Result INT;

    SELECT @Result = COUNT(id)
    FROM Roles
    LEFT JOIN dbo.Cast C on Roles.id = C.role_id
    WHERE role_id IS NULL;

    RETURN @Result;
END
GO;

SELECT dbo.fnAvailableRoles() AS 'Available Roles';

DROP FUNCTION IF EXISTS fnAvailableRoles;
GO;

-- Task: Get analysis by salary
CREATE FUNCTION dbo.fnActorsSalaryBetween(@lo INT, @hi INT)
RETURNS TABLE AS
RETURN (
    SELECT last_name, first_name, salary
    FROM Actors
    INNER JOIN dbo.Contracts on Actors.id = Contracts.actor_id
    WHERE salary BETWEEN @lo AND @hi
);
GO;

SELECT * FROM dbo.fnActorsSalaryBetween(0, 10000);

DROP FUNCTION IF EXISTS fnActorsSalaryBetween;