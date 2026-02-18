USE DBTheater
GO;

-- Actors Table (pseudonyms)
CREATE TABLE Actors
(
    id INT IDENTITY(1, 1),
    first_name VARCHAR(45) NOT NULL,
    last_name VARCHAR(45) NOT NULL,
    total_experience REAL NOT NULL DEFAULT 0,
    awards VARCHAR(255),

    CONSTRAINT PK_Actors PRIMARY KEY(id),
    CONSTRAINT CK_Actors_Experience CHECK(total_experience >= 0),
)
GO;

-- Contracts Table
CREATE TABLE Contracts
(
    id INT IDENTITY(1, 1),
    salary MONEY NOT NULL DEFAULT 0,
    hiring_date SMALLDATETIME NOT NULL DEFAULT GETDATE(),

    actor_id INT UNIQUE NOT NULL,

    CONSTRAINT PK_Contracts PRIMARY KEY(id),
    CONSTRAINT CK_Contract_Salary CHECK(salary >= 0),

    CONSTRAINT FK_Actors_Contracts FOREIGN KEY(actor_id) REFERENCES Actors(id)
        ON DELETE CASCADE ON UPDATE CASCADE
)
GO;

-- Shows Table
CREATE TABLE Shows
(
    id INT IDENTITY(1, 1),
    title VARCHAR(50) NOT NULL,
    budget MONEY NOT NULL DEFAULT 0,
    date SMALLDATETIME NOT NULL DEFAULT GETDATE(),

    CONSTRAINT PK_Shows PRIMARY KEY(id),
    CONSTRAINT CK_Show_Budget CHECK(budget >= 0)
)
GO;

-- Roles Table
CREATE TABLE Roles
(
    id INT IDENTITY(1, 1),
    title VARCHAR(50) NOT NULL,

    show_id INT NOT NULL,

    CONSTRAINT PK_Roles PRIMARY KEY(id),

    CONSTRAINT FK_Shows_Roles FOREIGN KEY(show_id) REFERENCES Shows(id)
        ON DELETE CASCADE ON UPDATE CASCADE
)
GO;

-- Cast Table (many-to-many additional table for Roles and Actors)
CREATE TABLE Cast
(
    actor_id INT,
    role_id INT,

    CONSTRAINT PK_Cast PRIMARY KEY (actor_id, role_id),

    CONSTRAINT FK_Roles_Cast FOREIGN KEY(role_id) REFERENCES Roles(id)
        ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT FK_Actors_Cast FOREIGN KEY(actor_id) REFERENCES Actors(id)
        ON DELETE CASCADE ON UPDATE CASCADE
)
GO;

-- UserTypes Table (for back-end)
CREATE TABLE UserTypes
(
    id   INT IDENTITY (1, 1),
    type VARCHAR(30) NOT NULL,

    CONSTRAINT PK_UserTypes PRIMARY KEY (id),
    CONSTRAINT UQ_Type UNIQUE (type)
)
GO;

-- Users Table (sensitive data)
CREATE TABLE Users
(
    id         INT IDENTITY (1, 1),
    first_name VARCHAR(30)      NOT NULL,
    patronymic VARCHAR(30),
    last_name  VARCHAR(30)      NOT NULL,
    type_id    INT              NOT NULL,
    email      VARCHAR(50),
    phone      VARCHAR(15),

    password   VARCHAR(64)      NOT NULL,
    salt       UNIQUEIDENTIFIER NOT NULL,

    contract_id INT,

    CONSTRAINT PK_Users PRIMARY KEY (id),
    CONSTRAINT UQ_User_Contract UNIQUE (contract_id),

    CONSTRAINT FK_User_Contract FOREIGN KEY(contract_id) REFERENCES Contracts(id)
        ON DELETE NO ACTION ON UPDATE CASCADE,
    CONSTRAINT FK_User_Role FOREIGN KEY(type_id) REFERENCES UserTypes(id)
        ON DELETE NO ACTION ON UPDATE CASCADE
)
GO;