USE DBTheater;
GO;

-- Creating global temporary tables for Lab Work.

CREATE PROCEDURE dbo.spCreateTempTables
AS
BEGIN
    CREATE TABLE ##Actors
    (
        id               INT,
        first_name       NVARCHAR(45) NOT NULL,
        last_name        NVARCHAR(45) NOT NULL,
        total_experience REAL         NOT NULL DEFAULT 0,
        awards           NVARCHAR(255),

        CONSTRAINT PK_Actors PRIMARY KEY (id),
        CONSTRAINT CK_Actors_Experience CHECK (total_experience >= 0),
    );

    CREATE TABLE ##Contracts
    (
        id          INT,
        salary      MONEY         NOT NULL DEFAULT 0,
        hiring_date SMALLDATETIME NOT NULL DEFAULT GETDATE(),

        actor_id    INT UNIQUE    NOT NULL,

        CONSTRAINT PK_Contracts PRIMARY KEY (id),
        CONSTRAINT CK_Contract_Salary CHECK (salary >= 0),

        CONSTRAINT FK_Actors_Contracts FOREIGN KEY (actor_id) REFERENCES Actors (id)
            ON DELETE CASCADE ON UPDATE CASCADE
    );

    CREATE TABLE ##Shows
    (
        id     INT,
        title  NVARCHAR(50)  NOT NULL,
        budget MONEY         NOT NULL DEFAULT 0,
        date   SMALLDATETIME NOT NULL DEFAULT GETDATE(),

        CONSTRAINT PK_Shows PRIMARY KEY (id),
        CONSTRAINT CK_Show_Budget CHECK (budget >= 0)
    );

    CREATE TABLE ##Roles
    (
        id      INT,
        title   NVARCHAR(50) NOT NULL,

        show_id INT          NOT NULL,

        CONSTRAINT PK_Roles PRIMARY KEY (id),

        CONSTRAINT FK_Shows_Roles FOREIGN KEY (show_id) REFERENCES Shows (id)
            ON DELETE CASCADE ON UPDATE CASCADE
    );

    CREATE TABLE ##Cast
    (
        actor_id INT,
        role_id  INT,

        CONSTRAINT PK_Cast PRIMARY KEY (actor_id, role_id),

        CONSTRAINT FK_Roles_Cast FOREIGN KEY (role_id) REFERENCES Roles (id)
            ON DELETE CASCADE ON UPDATE CASCADE,
        CONSTRAINT FK_Actors_Cast FOREIGN KEY (actor_id) REFERENCES Actors (id)
            ON DELETE CASCADE ON UPDATE CASCADE
    );
END;
GO;