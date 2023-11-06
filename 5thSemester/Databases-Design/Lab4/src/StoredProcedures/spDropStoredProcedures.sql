USE DBTheater;
GO;

-- Drop already created stored procedures.

CREATE PROCEDURE spDropStoredProcedures
AS
BEGIN
    DROP PROCEDURE IF EXISTS spCreateTempTables;
    DROP PROCEDURE IF EXISTS spDropTempTables;
    DROP PROCEDURE IF EXISTS spFulfillTempTables;
    DROP PROCEDURE IF EXISTS spDropStoredProcedures;
END;
GO;