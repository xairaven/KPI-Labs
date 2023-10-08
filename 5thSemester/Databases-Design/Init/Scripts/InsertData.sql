USE DBTheater;
GO;

---------- Actors ----------
CREATE TABLE #BulkActors
(
    first_name NVARCHAR(45) NOT NULL,
    last_name NVARCHAR(45) NOT NULL,
    total_experience REAL NOT NULL DEFAULT 0,
    awards NVARCHAR(255)
);
BULK INSERT #BulkActors
FROM 'D:\Programming\DBProjects\KPI-Labs\Init\Data\Actors.csv'
WITH (
    FIELDTERMINATOR = ',',
    ROWTERMINATOR = '\n',
    FIRSTROW = 2
);
INSERT INTO Actors(first_name, last_name, total_experience, awards)
SELECT first_name, last_name, total_experience, awards FROM #BulkActors;
DROP TABLE IF EXISTS #BulkActors;
GO;

---------- Contracts ----------
CREATE TABLE #BulkContracts
(
    salary MONEY NOT NULL DEFAULT 0,
    hiring_date SMALLDATETIME NOT NULL DEFAULT GETDATE(),

    actor_id INT UNIQUE NOT NULL
);
BULK INSERT #BulkContracts
FROM 'D:\Programming\DBProjects\KPI-Labs\Init\Data\Contracts.csv'
WITH (
    FIELDTERMINATOR = ',',
    ROWTERMINATOR = '\n',
    FIRSTROW = 2
);
INSERT INTO Contracts(salary, hiring_date, actor_id)
SELECT salary, CAST(hiring_date as date), actor_id FROM #BulkContracts;
DROP TABLE IF EXISTS #BulkContracts;
GO;

---------- Shows ----------
CREATE TABLE #BulkShows
(
    title NVARCHAR(50) NOT NULL,
    budget MONEY NOT NULL DEFAULT 0,
    date SMALLDATETIME NOT NULL DEFAULT GETDATE()
);
BULK INSERT #BulkShows
FROM 'D:\Programming\DBProjects\KPI-Labs\Init\Data\Shows.csv'
WITH (
    FIELDTERMINATOR = ',',
    ROWTERMINATOR = '\n',
    FIRSTROW = 2
);
INSERT INTO Shows(title, budget, date)
SELECT title, budget, date FROM #BulkShows;
DROP TABLE IF EXISTS #BulkShows;
GO;

---------- Roles ----------
CREATE TABLE #BulkRoles
(
    title NVARCHAR(50) NOT NULL,
    show_id INT NOT NULL
);
BULK INSERT #BulkRoles
FROM 'D:\Programming\DBProjects\KPI-Labs\Init\Data\Roles.csv'
WITH (
    FIELDTERMINATOR = ',',
    ROWTERMINATOR = '\n',
    FIRSTROW = 2
);
INSERT INTO Roles(title, show_id)
SELECT title, show_id FROM #BulkRoles;
DROP TABLE IF EXISTS #BulkRoles;
GO;

---------- Cast ----------
CREATE TABLE #BulkCast
(
    actor_id INT,
    role_id INT,
);
BULK INSERT #BulkCast
FROM 'D:\Programming\DBProjects\KPI-Labs\Init\Data\Cast.csv'
WITH (
    FIELDTERMINATOR = ',',
    ROWTERMINATOR = '\n',
    FIRSTROW = 2
);
INSERT INTO Cast(actor_id, role_id)
SELECT actor_id, role_id FROM #BulkCast;
DROP TABLE IF EXISTS #BulkCast;