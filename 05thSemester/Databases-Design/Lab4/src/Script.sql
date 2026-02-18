USE tempdb;
GO;

-- Task 1: Transaction commit, rollback, savepoint.

DBTheater.dbo.spCreateTempTables;
GO;
DBTheater.dbo.spFulfillTempTables;
GO;

SELECT * FROM ##Actors;
SELECT * FROM ##Contracts;

BEGIN TRANSACTION
UPDATE ##Actors
SET awards = 'Oscar'
WHERE id = 1;

SAVE TRANSACTION GiveAward

UPDATE ##Contracts
SET salary = salary + 100
WHERE actor_id = 1;

ROLLBACK
ROLLBACK TRANSACTION GiveAward
COMMIT
GO;

SELECT @@TRANCOUNT;

DBTheater.dbo.spDropTempTables;


-- Task 2: Show a feature of transactions: blocking access to data.

DBTheater.dbo.spCreateTempTables;
GO;
DBTheater.dbo.spFulfillTempTables;
GO;

BEGIN TRANSACTION

UPDATE ##Actors
SET awards = 'Oscar'
WHERE id = 1;

master.dbo.spShowTranLocks tempdb;

ROLLBACK;
COMMIT;
GO;

SELECT @@TRANCOUNT;

DBTheater.dbo.spDropTempTables;



-- Task 3: Demonstrate how other data blocking commands work (lock / unlock, etc.).

DBTheater.dbo.spCreateTempTables;
GO;
DBTheater.dbo.spFulfillTempTables;
GO;

BEGIN TRANSACTION

SELECT * FROM ##Actors WITH (TABLOCKX);

master.dbo.spShowTranLocks tempdb;

ROLLBACK;
COMMIT;
GO;

SELECT @@TRANCOUNT;

DBTheater.dbo.spDropTempTables;