USE DPGI
GO;

DROP TABLE IF EXISTS #BulkData
GO;

CREATE TABLE #BulkData
(
    isbn             VARCHAR(30),
    title            VARCHAR(255),
    authors          VARCHAR(255),
    publisher        VARCHAR(255),
    publication_year SMALLINT
)
GO;

BULK INSERT #BulkData
    FROM 'D:\Programming\KPI-DPGI\Lab5\Database\Data\Data.csv'
    WITH (
    FIELDTERMINATOR = ',',
    ROWTERMINATOR = '\n',
    FIRSTROW = 2
    );
INSERT INTO dbo.Publishers (name)
SELECT DISTINCT publisher
FROM #BulkData
GO;

INSERT INTO dbo.Books (isbn, title, authors, publication_year, publisher_code)
SELECT bd.isbn,
       bd.title,
       bd.authors,
       bd.publication_year,
       pub.id AS publisher_code
FROM #BulkData AS bd
INNER JOIN dbo.Publishers AS pub
ON bd.publisher = pub.name;

DROP TABLE IF EXISTS #BulkData
GO;