USE DBTheater;
GO;

-- Drop already created Temp Tables.

CREATE PROCEDURE spDropTempTables
AS
BEGIN
    DROP TABLE IF EXISTS ##Actors;
    DROP TABLE IF EXISTS ##Cast;
    DROP TABLE IF EXISTS ##Contracts;
    DROP TABLE IF EXISTS ##Roles;
    DROP TABLE IF EXISTS ##Shows;
END;
GO;