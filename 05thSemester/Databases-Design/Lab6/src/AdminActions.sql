-- Create Login
-- NB! Connection by credentials, without IP
-- NB! Logon triggers
CREATE LOGIN AlexKovalov WITH PASSWORD = 'password';
GO;

-- Rename AlexKovalov
ALTER LOGIN AlexKovalov
WITH NAME = Alex;

-- Create User
USE DBTheater;
CREATE USER AlexUser FOR LOGIN Alex;
GO;

USE master;
CREATE USER AlexUser FOR LOGIN Alex;
GO;

USE tempdb;
CREATE USER AlexUser FOR LOGIN Alex;
GO;

-- Add user to the role
ALTER SERVER ROLE [sysadmin] ADD MEMBER Alex;
GO;

-- Add role to AlexUser
USE master;
ALTER ROLE db_owner ADD MEMBER AlexUser;
GO;

USE DBTheater;
ALTER ROLE db_datareader ADD MEMBER AlexUser;
GO;

USE tempdb;
ALTER ROLE db_owner ADD MEMBER AlexUser;
GO;

-- Grant insert permission to AlexUser
USE DBTheater;
REVOKE INSERT, DELETE ON Actors TO AlexUser;
GRANT INSERT, DELETE ON Actors TO AlexUser;
DENY INSERT, DELETE ON Actors TO AlexUser;
GO;

-- Drop user from the server role
ALTER SERVER ROLE [sysadmin] DROP MEMBER Alex;
GO;

-- Take the role from user AlexUser
USE master;
ALTER ROLE db_owner DROP MEMBER AlexUser;
GO;

USE DBTheater;
ALTER ROLE db_datareader DROP MEMBER AlexUser;
GO;

USE tempdb;
ALTER ROLE db_owner DROP MEMBER AlexUser;
GO;

-- Drop users
USE DBTheater;
DROP USER AlexUser;
GO;

USE master;
DROP USER AlexUser;
GO;

USE tempdb;
DROP USER AlexUser;
GO;

-- Drop Login
USE master;
DROP LOGIN Alex;
GO;