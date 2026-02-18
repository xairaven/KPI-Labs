-- Execute all instructions from Init.sql, check if spShowTranLocks exists
USE tempdb;
GO;

ALTER DATABASE tempdb
SET ALLOW_SNAPSHOT_ISOLATION OFF;
SET TRANSACTION ISOLATION LEVEL READ COMMITTED;

-- Check current isolation level
SELECT CASE
          WHEN transaction_isolation_level = 1
             THEN 'READ UNCOMMITTED'
          WHEN transaction_isolation_level = 2
               AND is_read_committed_snapshot_on = 1
             THEN 'READ COMMITTED SNAPSHOT'
          WHEN transaction_isolation_level = 2
               AND is_read_committed_snapshot_on = 0 THEN 'READ COMMITTED'
          WHEN transaction_isolation_level = 3
             THEN 'REPEATABLE READ'
          WHEN transaction_isolation_level = 4
             THEN 'SERIALIZABLE'
          WHEN transaction_isolation_level = 5
             THEN 'SNAPSHOT'
          ELSE NULL
       END AS [TRANSACTION ISOLATION LEVEL]
FROM   sys.dm_exec_sessions AS s
       CROSS JOIN sys.databases AS d
WHERE  session_id = @@SPID
  AND  d.database_id = DB_ID();

-- Check locks
master.dbo.spShowTranLocks tempdb;

-- Create 2 users and demonstrate their parallel work.
BEGIN TRANSACTION
INSERT INTO ##Actors(id, first_name, last_name, total_experience)
VALUES (501, 'Alex', 'Kovalov', 12);
COMMIT;
ROLLBACK;

SELECT * FROM ##Actors ORDER BY id OFFSET 495 ROWS;
SELECT @@TRANCOUNT;

-- Write transactions that are blocking something (rows, tables etc.).
-- Transaction with update and rollback
---- DIRTY READS EXAMPLE
master.dbo.spShowTranLocks tempdb;

BEGIN TRANSACTION
UPDATE ##Actors
SET awards = 'Oscar'
WHERE id = 498;
COMMIT;
ROLLBACK;

SELECT * FROM ##Actors ORDER BY id OFFSET 495 ROWS;
SELECT @@TRANCOUNT;

-- Transaction with SAVEPOINT and subtransactions after it and rollover to this point
---- WITH NON-REPEATABLE READS:
master.dbo.spShowTranLocks tempdb;

BEGIN TRANSACTION
SET TRANSACTION ISOLATION LEVEL READ COMMITTED
    SELECT * FROM ##Actors WHERE ID = 498;
    SAVE TRANSACTION UpdateHere;
    SELECT * FROM ##Actors WHERE ID = 498;
ROLLBACK TRANSACTION UpdateHere;
ROLLBACK;
COMMIT;

SELECT * FROM ##Actors ORDER BY id OFFSET 495 ROWS;
SELECT @@TRANCOUNT;

---- WITHOUT NON-REPEATABLE READS:
master.dbo.spShowTranLocks tempdb;

BEGIN TRANSACTION
SET TRANSACTION ISOLATION LEVEL REPEATABLE READ
    SELECT * FROM ##Actors WHERE ID = 498;
    SAVE TRANSACTION UpdateHere;
    SELECT * FROM ##Actors WHERE ID = 498;
ROLLBACK TRANSACTION UpdateHere;
ROLLBACK;
COMMIT;

SELECT * FROM ##Actors ORDER BY id OFFSET 495 ROWS;
SELECT @@TRANCOUNT;

-- Locking the table for reading, trying to select data and unlocking the table.
-- MINUS
-- Phantom reads
---- WITHOUT SERIALIZABLE:
master.dbo.spShowTranLocks tempdb;

BEGIN TRANSACTION
SET TRANSACTION ISOLATION LEVEL REPEATABLE READ
    SELECT * FROM ##Actors WHERE ID BETWEEN 498 AND 505;
    SAVE TRANSACTION UpdateHere;
    SELECT * FROM ##Actors WHERE ID BETWEEN 498 AND 505;
ROLLBACK TRANSACTION UpdateHere;
ROLLBACK;
COMMIT;

SELECT * FROM ##Actors ORDER BY id OFFSET 495 ROWS;
SELECT @@TRANCOUNT;

---- WITH SERIALIZABLE:
master.dbo.spShowTranLocks tempdb;

BEGIN TRANSACTION
SET TRANSACTION ISOLATION LEVEL SERIALIZABLE
    SELECT * FROM ##Actors WHERE ID BETWEEN 498 AND 505;
    SAVE TRANSACTION UpdateHere;
    SELECT * FROM ##Actors WHERE ID BETWEEN 498 AND 505;
ROLLBACK TRANSACTION UpdateHere;
ROLLBACK;
COMMIT;

SELECT * FROM ##Actors ORDER BY id OFFSET 495 ROWS;
SELECT @@TRANCOUNT;

-- Set isolation level to READ UNCOMMITED
-- 1 TRAN without explicit isolation level. Select updates. Rollback
---- PREVIOUS EXAMPLES

-- SNAPSHOT ISOLATION LEVEL
ALTER DATABASE tempdb
SET ALLOW_SNAPSHOT_ISOLATION ON;
master.dbo.spShowTranLocks tempdb;

BEGIN TRANSACTION
SET TRANSACTION ISOLATION LEVEL SNAPSHOT
    SELECT * FROM ##Actors WHERE ID BETWEEN 498 AND 505;
COMMIT;
ROLLBACK;
SELECT @@TRANCOUNT;

SELECT * FROM tempdb.sys.dm_tran_version_store;
SELECT * FROM tempdb.sys.dm_tran_version_store_space_usage;

-- READ COMMITED SNAPSHOT is the same, but with X-Locks

-- Execute all instructions from Final.sql