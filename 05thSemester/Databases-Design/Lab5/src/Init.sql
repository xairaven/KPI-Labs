DBTheater.dbo.spDropTempTables;
GO;
DBTheater.dbo.spCreateTempTables;
GO;
DBTheater.dbo.spFulfillTempTables;
GO;

-- Create Login
CREATE LOGIN Alex WITH PASSWORD = 'password';
GO;

USE DBTheater;
CREATE USER AlexUser FOR LOGIN Alex;
ALTER ROLE db_owner ADD MEMBER AlexUser;
GO;

USE master;
CREATE USER AlexUser FOR LOGIN Alex;
ALTER ROLE db_owner ADD MEMBER AlexUser;
GO;

USE tempdb;
CREATE USER AlexUser FOR LOGIN Alex;
ALTER ROLE db_owner ADD MEMBER AlexUser;
GO;