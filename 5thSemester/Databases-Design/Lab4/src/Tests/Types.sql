CREATE DATABASE TransactionsTests;
GO;

USE TransactionsTests;
GO;

-------- EXPLICIT TRANSACTIONS VS AUTOCOMMIT. BENCHMARK --------

-- Creating table with some data --
DROP TABLE IF EXISTS Data;
CREATE TABLE Data
(
	ID INT NOT NULL,
	Text VARCHAR(50) NULL,

    CONSTRAINT PK_Types PRIMARY KEY (Id)
);
GO;



-- Global (script) variables for logging --
DECLARE
	@Id INT = 1,
	@StartTime DATETIME,
	@NumOfWrites BIGINT,
	@NumOfBytesWritten BIGINT;

SELECT @NumOfWrites = num_of_writes, @NumOfBytesWritten = num_of_bytes_written
FROM sys.dm_io_virtual_file_stats(db_id(),2);

SELECT @StartTime = getDate();

-- Auto-committed transactions --
WHILE @Id < 1000
BEGIN
	INSERT INTO dbo.Data(ID, Text)
	VALUES (@Id, 'A');

	UPDATE dbo.Data
	SET Text = 'B'
	WHERE ID = @Id;

	DELETE FROM dbo.Data
	WHERE ID = @Id;

	SET @Id += 1;
END;

-- Log file statistics --
SELECT
	DATEDIFF(millisecond, @StartTime, getDate()) as [Exec Time ms: Autocommited],
	s.num_of_writes - @NumOfWrites as [Number of writes],
	(s.num_of_bytes_written - @NumOfBytesWritten) / 1024 as [Kilobytes written (KB)]
FROM
	sys.dm_io_virtual_file_stats(db_id(), 2) s;
GO;



-- Resetting global (script) variables for logging --
DECLARE
	@Id INT = 1,
	@StartTime DATETIME,
	@NumOfWrites BIGINT,
	@NumOfBytesWritten BIGINT

SELECT @NumOfWrites = num_of_writes, @NumOfBytesWritten = num_of_bytes_written
FROM sys.dm_io_virtual_file_stats(db_id(), 2);

SELECT @StartTime = getDate();

-- Explicit Transaction
WHILE @Id < 1000
BEGIN
	BEGIN TRANSACTION
		INSERT INTO dbo.Data(ID, Text)
	    VALUES (@Id, 'A');

	    UPDATE dbo.Data
	    SET Text = 'B'
	    WHERE ID = @Id;

	    DELETE FROM dbo.Data
	    WHERE ID = @Id;
	COMMIT
	SET @Id += 1;
END;

-- Log file statistics --
SELECT
	DATEDIFF(millisecond, @StartTime, getDate()) as [Exec Time ms: Explicit],
	s.num_of_writes - @NumOfWrites as [Number of writes],
	(s.num_of_bytes_written - @NumOfBytesWritten) / 1024 as [Kilobytes written (KB)]
FROM
	sys.dm_io_virtual_file_stats(db_id(), 2) AS s;
GO;

-- SET IMPLICIT_TRANSACTIONS ON;
-- Same as EXPLICIT

-- 10000:
-- Auto-committed.
-- Exec Time ms: 13087
-- Number of writes: 30004
-- Kilobytes written (KB): 120644

-- Explicit.
-- Exec Time ms: 5006
-- Number of writes: 10000
-- Kilobytes written (KB): 40624

EXEC SP_WHO;

DROP DATABASE IF EXISTS TransactionsTests;
GO;