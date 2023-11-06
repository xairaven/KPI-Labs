USE master;
GO;

-- Show info about concurrency locks.

CREATE PROCEDURE spShowTranLocks
    @DatabaseName SYSNAME
AS
BEGIN
    DECLARE @SqlQuery NVARCHAR(MAX);

    SET @SqlQuery = N'SELECT request_session_id AS [ID],
           resource_type AS [Resource Type],
           request_type AS [Request Type],
           request_mode AS [Mode],
           request_owner_id AS [Owner ID],
           request_owner_type AS [Owner]
           FROM ' + QUOTENAME(@DatabaseName) + '.sys.dm_tran_locks';

    EXEC sp_executesql @SqlQuery;
END;
GO;

