USE DPGI
GO;

CREATE TABLE #BulkBooks
(
    isbn             VARCHAR(30),
    title            VARCHAR(255),
    authors          VARCHAR(255),
    publisher        VARCHAR(255),
    publication_year SMALLINT
)
BULK INSERT #BulkBooks
FROM 'D:\Programming\KPI-DPGI\Lab4\Database\Data\Data.csv'
WITH (
    FIELDTERMINATOR = ',',
    ROWTERMINATOR = '\n',
    FIRSTROW = 2
);
INSERT INTO dbo.Books
SELECT * FROM #BulkBooks;
DROP TABLE IF EXISTS #BulkBooks;
GO;