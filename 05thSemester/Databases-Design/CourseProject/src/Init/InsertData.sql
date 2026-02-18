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
FROM 'D:\Programming\DBProjects\CourseProject\Init\Data\Actors.csv'
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
FROM 'D:\Programming\DBProjects\CourseProject\Init\Data\Contracts.csv'
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
FROM 'D:\Programming\DBProjects\CourseProject\Init\Data\Shows.csv'
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
FROM 'D:\Programming\DBProjects\CourseProject\Init\Data\Roles.csv'
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
FROM 'D:\Programming\DBProjects\CourseProject\Init\Data\Cast.csv'
WITH (
    FIELDTERMINATOR = ',',
    ROWTERMINATOR = '\n',
    FIRSTROW = 2
);
INSERT INTO Cast(actor_id, role_id)
SELECT actor_id, role_id FROM #BulkCast;
DROP TABLE IF EXISTS #BulkCast;

---------- UserTypes ----------
INSERT INTO UserTypes(type)
VALUES ('Admin'),
       ('FinancialDirector'),
       ('Accountant'),
       ('Actor')


---------- Users ----------
INSERT INTO Users(first_name, patronymic, last_name, type_id,
                  email, phone, password, salt, contract_id)
VALUES ('Alex', 'Leonidovych', 'Dikovskyi', 4, 'alexdikovskyi@gmail.com',
        '+3809912312312', 'hr5Oo+f+_QR=', '0dc26fac-ad45-4823-ae7c-86a8148cb4f7', 1),
    ('Vladyslav', 'Valeriyovuch', 'Zhukovskyi', 4, 'vladzhuk@gmail.com',
        '+3809964378587', 'pT3PdSN.oXaJ', '28fbb576-bbe6-4f5e-870c-3826f560529d', 2),
    ('Artem', 'Vladyslavovuch', 'Kovalov', 4, 'artkovalov@gmail.com',
        '+3809936892367', 'hptzPr84F_DN', 'ec3df2cc-399a-455d-993a-28f2ff5966ed', 3),
    ('Danylo', 'Mykhailovych', 'Karkushevskyi', 4, 'dankurk@gmail.com',
        '+3809998527569', 'QYDJhx)W5X@W', '053574bd-102f-4bb6-8bbd-baba89f3d495', 4),
    ('Liza', 'Oleksiivna', 'Dikovska', 4, 'lizadikovska@gmail.com',
        '+3809919572495', 'T24Uhi(s:)xR', '039e15e8-5a2b-495c-b22e-008140144332', 5)