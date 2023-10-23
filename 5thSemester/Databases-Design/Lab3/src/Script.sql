USE DBTheater;
GO;

---------- FIRST TASK: Logging database events ----------
DROP TABLE IF EXISTS DDLLogs;
GO;

-- Create tables for logs.
CREATE TABLE DDLLogs (
    EventTime DATETIME,
    DB_User NVARCHAR(100),
    Schema_Name NVARCHAR(100),
    Event NVARCHAR(100),
    TSQL NVARCHAR(2000));
GO;

-- Trigger.
CREATE TRIGGER TR_DDL_Logs
ON DATABASE
FOR DDL_DATABASE_LEVEL_EVENTS
AS
DECLARE @data XML
SET @data = EVENTDATA()
INSERT DDLLogs (EventTime, DB_User, Schema_Name, Event, TSQL)
VALUES (GETDATE(),
   @data.value('(/EVENT_INSTANCE/LoginName)[1]', 'NVARCHAR(100)'),
   @data.value('(/EVENT_INSTANCE/SchemaName)[1]', 'NVARCHAR(100)'),
   @data.value('(/EVENT_INSTANCE/EventType)[1]', 'NVARCHAR(100)'),
   @data.value('(/EVENT_INSTANCE/TSQLCommand)[1]', 'NVARCHAR(2000)'));
GO;


-- Test the trigger.
CREATE TABLE TestTable(id INT);
DROP TABLE TestTable;
GO;

SELECT * FROM DDLLogs;
GO;

-- Drop the trigger
DROP TRIGGER IF EXISTS TR_DDL_Logs
ON DATABASE;
GO;

-- Drop table with logs;
DROP TABLE IF EXISTS DDLLogs;
GO;

---------- SECOND TASK: Restrict view updating ----------
DROP VIEW IF EXISTS vwActors
GO;

CREATE VIEW vwActors AS
SELECT last_name, first_name
FROM Actors;
GO;

SELECT * FROM vwActors;
GO;

-- Create trigger that blocks all updates
CREATE TRIGGER TR_vwActors_BlockUpdates
ON vwActors
INSTEAD OF INSERT, UPDATE, DELETE
AS
BEGIN
    RAISERROR('Updates to this view are not allowed.', 14, 1);
END;
GO;

-- Error Handling
BEGIN TRY
    -- Attempt an update operation on the restricted view
    UPDATE vwActors
    SET last_name = 'Value'
    WHERE last_name = 'Phi';
END TRY
BEGIN CATCH
    SELECT 'Error occurred: ' + ERROR_MESSAGE();
END CATCH;
GO;

DROP VIEW IF EXISTS vwActors
GO;

DROP TRIGGER IF EXISTS TR_vwActors_BlockUpdates
GO;