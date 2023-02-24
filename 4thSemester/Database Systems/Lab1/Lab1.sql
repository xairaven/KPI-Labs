CREATE DATABASE Lab1;
USE Lab1;

CREATE TABLE Actors (
	ID INT PRIMARY KEY AUTO_INCREMENT,
    Surname VARCHAR(20) NOT NULL,
	Name VARCHAR(20) NOT NULL,
	Age INT NOT NULL,
	CHECK (Age >= 18)
);

INSERT INTO Actors (Surname, Name, Age)
	VALUES
	('Kovalyov', 'Alex', 19),
	('Dikovskyi', 'Artem', 19);
 
LOAD DATA LOCAL INFILE 'D:/Docs/DB/Lab1/Data.txt'
	INTO TABLE Actors (Surname, Name, Age);
    
UPDATE Actors
	SET Age = 75
	WHERE (Surname = 'Dikovskyi' AND Id <> 0);
    
DELETE FROM Actors
	WHERE Surname = 'Dikovskyi' AND Id <> 0;
    
ALTER TABLE Actors
	ADD Country VARCHAR(15) NOT NULL;
    
UPDATE Actors
	SET Id = -1
	WHERE Id <> 0;
    
UPDATE Actors
	SET Age = 5
	WHERE Surname = 'Dikovskiy' AND Id <> 0 ;
 
UPDATE Actors
	SET Age = NULL
	WHERE Id <> 0;
    
DROP TABLE Actors;
DROP DATABASE Lab1;