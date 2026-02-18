-- Authentication mode
SELECT CASE SERVERPROPERTY('IsIntegratedSecurityOnly')
          WHEN 1 THEN 'Windows Authentication Only'
          WHEN 0 THEN 'Windows and SQL Server Authentication'
       END as [Authentication Mode];

-- Check all server members
SELECT principal_id AS [ID],
       name AS [Name],
       type_desc AS [Type],
       create_date AS [Date of creation]
FROM sys.server_principals
WHERE type_desc LIKE 'SQL_LOGIN';

-- Check all DB members
SELECT principal_id AS [ID],
       name AS [Name],
       type_desc AS [Type],
       default_schema_name AS [Default Schema],
       create_date AS [Date of creation]
FROM sys.database_principals
WHERE type_desc LIKE 'SQL_USER';

-- Check subjects of all server roles
SELECT [Roles].name AS [Server Role],
    server_role_members.member_principal_id AS MemberID,
    members.name AS Member
FROM sys.server_role_members AS server_role_members
INNER JOIN sys.server_principals AS [Roles]
    ON server_role_members.role_principal_id = roles.principal_id
LEFT JOIN sys.server_principals AS members
    ON server_role_members.member_principal_id = members.principal_id;
GO;

-- Check subjects of all Database roles
USE tempdb;
SELECT roles.name AS [DB Role],
       db_role_members.member_principal_id AS MemberID,
       members.name AS Member
FROM sys.database_role_members AS db_role_members
JOIN sys.database_principals AS roles
    ON db_role_members.role_principal_id = roles.principal_id
JOIN sys.database_principals AS members
    ON db_role_members.member_principal_id = members.principal_id;
GO;

-- Check users and roles permissions
USE tempdb;
SELECT Principals.name [Name or Role],
       Principals.type_desc [Type Desc],
       Permissions.permission_name [Permission Name],
       Permissions.state_desc [State Desc],
       Permissions.class_desc AS [Short Desc],
       object_name(Permissions.major_id) AS [Description]
FROM sys.database_principals AS Principals
LEFT JOIN sys.database_permissions Permissions
ON Permissions.grantee_principal_id = Principals.principal_id
WHERE Principals.name LIKE 'AlexUser';
GO;