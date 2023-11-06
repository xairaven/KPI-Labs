-- Execute all instructions from Init.sql
USE tempdb;
GO;

-- Create 2 users and demonstrate their parallel work.
BEGIN TRANSACTION
UPDATE ##Actors
SET awards = 'Oscar'
WHERE id = 500;
COMMIT;
ROLLBACK;

SELECT * FROM ##Actors ORDER BY id OFFSET 495 ROWS;
SELECT @@TRANCOUNT;

-- Write transactions that are blocking something (rows, tables etc.).
-- Transaction with update and rollback
---- WITH DIRTY READS:
BEGIN TRANSACTION
SET TRANSACTION ISOLATION LEVEL READ UNCOMMITTED
SELECT * FROM ##Actors WHERE Id BETWEEN 495 AND 501; -- W/O S LOCK
-- Doing something with dat data... ... ... ... ...
COMMIT;
ROLLBACK;

SELECT @@TRANCOUNT;

---- WITHOUT DIRTY READS:
BEGIN TRANSACTION
SET TRANSACTION ISOLATION LEVEL READ COMMITTED -- + S LOCK
SELECT * FROM ##Actors WHERE Id BETWEEN 495 AND 501;
-- Doing something with dat data... ... ... ... ...
COMMIT;
ROLLBACK;

SELECT @@TRANCOUNT;

-- Transaction with SAVEPOINT and subtransactions after it and rollover to this point
---- NON-REPEATABLE READS EXAMPLE
BEGIN TRANSACTION
    UPDATE ##Actors
    SET awards = 'SuperAward'
    WHERE id = 498;
ROLLBACK;
COMMIT;

SELECT @@TRANCOUNT;

-- Locking the table for reading, trying to select data and unlocking the table.
-- MINUS
-- Phantom reads EXAMPLE
BEGIN TRANSACTION
    INSERT INTO ##Actors(id, first_name, last_name)
    VALUES (501, 'Alex', 'Kovalov');
ROLLBACK;
COMMIT;

SELECT @@TRANCOUNT;

-- Set isolation level to READ UNCOMMITED
-- 1 TRAN without explicit isolation level. Select updates. Rollback
---- PREVIOUS EXAMPLES

-- SNAPSHOT ISOLATION LEVEL
BEGIN TRANSACTION
    INSERT INTO ##Actors(id, first_name, last_name)
    VALUES (504, 'Artem', 'Dikovskyi');
COMMIT;
ROLLBACK;

SELECT @@TRANCOUNT;
SELECT * FROM ##Actors WHERE ID BETWEEN 498 AND 505;
-- Execute all instructions from Final.sql