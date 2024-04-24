CREATE DATABASE DPGI
GO;

USE DPGI
GO;

CREATE TABLE dbo.Publishers
(
    id   INT IDENTITY (1, 1),
    name VARCHAR(255) NOT NULL,
    
    PRIMARY KEY (id)
)
GO;

CREATE TABLE dbo.Books
(
    isbn             VARCHAR(30)  NOT NULL,
    title            VARCHAR(255) NOT NULL,
    authors          VARCHAR(255) NOT NULL,
    publisher_code   INT          NOT NULL,
    publication_year SMALLINT,

    PRIMARY KEY (isbn),

    CONSTRAINT FK_PublishersCode
        FOREIGN KEY (publisher_code) REFERENCES dbo.Publishers (id)
)
GO;