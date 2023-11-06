DBTheater.dbo.spDropTempTables;
GO;

ALTER DATABASE tempdb
SET ALLOW_SNAPSHOT_ISOLATION OFF;
SET TRANSACTION ISOLATION LEVEL READ COMMITTED;

USE master;
ALTER ROLE db_owner DROP MEMBER AlexUser;
DROP USER AlexUser;
GO;

USE DBTheater;
ALTER ROLE db_owner DROP MEMBER AlexUser;
DROP USER AlexUser;
GO;

USE tempdb;
ALTER ROLE db_datareader DROP MEMBER AlexUser;
DROP USER AlexUser;
GO;

USE master;
DROP LOGIN Alex;
GO;

DBTheater.dbo.spDropStoredProcedures;