CREATE DATABASE DPGI
GO;

USE DPGI
GO;

CREATE TABLE dbo.Books
(
    isbn             VARCHAR(30),
    title            VARCHAR(255),
    authors          VARCHAR(255),
    publisher        VARCHAR(255),
    publication_year SMALLINT
)
GO;