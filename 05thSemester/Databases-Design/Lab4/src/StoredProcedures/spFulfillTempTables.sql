USE DBTheater;
GO;

-- Fulfilling temp tables from main tables.

CREATE PROCEDURE spFulfillTempTables
AS
BEGIN
    IF
    (SELECT COUNT(*) FROM tempdb.INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = '##Actors') > 0
    BEGIN
        INSERT INTO ##Actors
        SELECT * FROM Actors
    END;

    IF
    (SELECT COUNT(*) FROM tempdb.INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = '##Cast') > 0
    BEGIN
        INSERT INTO ##Cast
        SELECT * FROM Cast
    END;

    IF
    (SELECT COUNT(*) FROM tempdb.INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = '##Contracts') > 0
    BEGIN
        INSERT INTO ##Contracts
        SELECT * FROM Contracts
    END;

    IF
    (SELECT COUNT(*) FROM tempdb.INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = '##Roles') > 0
    BEGIN
        INSERT INTO ##Roles
        SELECT * FROM Roles
    END;

    IF
    (SELECT COUNT(*) FROM tempdb.INFORMATION_SCHEMA.TABLES WHERE TABLE_NAME = '##Shows') > 0
    BEGIN
        INSERT INTO ##Shows
        SELECT * FROM Shows
    END;
END;
GO;