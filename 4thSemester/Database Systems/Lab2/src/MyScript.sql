SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0;
SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0;
SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION';

CREATE SCHEMA IF NOT EXISTS `TheaterActors` DEFAULT CHARACTER SET utf8 ;
USE `TheaterActors` ;

CREATE TABLE IF NOT EXISTS `TheaterActors`.`Actors` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `first_name` VARCHAR(45) NOT NULL,
  `last_name` VARCHAR(45) NOT NULL,
  `hiring_date` DATETIME NULL DEFAULT (CURDATE()),
  `years_of_experience` INT NULL DEFAULT (FLOOR(datediff(CURDATE(), hiring_date) / 365)),
  `awards` VARCHAR(255) NULL DEFAULT '',
  `spectacle_counter` INT NULL DEFAULT 0,
  PRIMARY KEY (`id`))
ENGINE = InnoDB;

CREATE TABLE IF NOT EXISTS `TheaterActors`.`Shows` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `title` VARCHAR(255) NOT NULL,
  `budget` DECIMAL NOT NULL,
  `date` DATETIME NULL DEFAULT (CURDATE()),
  PRIMARY KEY (`id`))
ENGINE = InnoDB;

CREATE TABLE IF NOT EXISTS `TheaterActors`.`Roles` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `title` VARCHAR(50) NOT NULL,
  PRIMARY KEY (`id`))
ENGINE = InnoDB;

CREATE TABLE IF NOT EXISTS `TheaterActors`.`Contracts` (
  `id` INT NOT NULL AUTO_INCREMENT,
  `base_salary` DECIMAL NOT NULL,
  `salary` DECIMAL NOT NULL DEFAULT (base_salary + bonus),
  `bonus` DECIMAL NOT NULL DEFAULT 0,
  `Actors_id` INT NOT NULL,
  PRIMARY KEY (`id`),
  INDEX `fk_Contracts_Actors_idx` (`Actors_id` ASC) VISIBLE,
  CONSTRAINT `fk_Contracts_Actors`
    FOREIGN KEY (`Actors_id`)
    REFERENCES `TheaterActors`.`Actors` (`id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
ENGINE = InnoDB;

CREATE TABLE IF NOT EXISTS `TheaterActors`.`AdditionalTable` (
  `Roles_id` INT NOT NULL,
  `Shows_id` INT NOT NULL,
  `Actors_id` INT NOT NULL,
  INDEX `fk_AdditionalTable_Roles1_idx` (`Roles_id` ASC) VISIBLE,
  INDEX `fk_AdditionalTable_Shows1_idx` (`Shows_id` ASC) VISIBLE,
  INDEX `fk_AdditionalTable_Actors1_idx` (`Actors_id` ASC) VISIBLE,
  CONSTRAINT `fk_AdditionalTable_Roles1`
    FOREIGN KEY (`Roles_id`)
    REFERENCES `TheaterActors`.`Roles` (`id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  CONSTRAINT `fk_AdditionalTable_Shows1`
    FOREIGN KEY (`Shows_id`)
    REFERENCES `TheaterActors`.`Shows` (`id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE,
  CONSTRAINT `fk_AdditionalTable_Actors1`
    FOREIGN KEY (`Actors_id`)
    REFERENCES `TheaterActors`.`Actors` (`id`)
    ON DELETE CASCADE
    ON UPDATE CASCADE)
ENGINE = InnoDB;

SET SQL_MODE=@OLD_SQL_MODE;
SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS;
SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS;

DELIMITER //
CREATE TRIGGER update_played_roles_trigger
AFTER INSERT
ON additionaltable
FOR EACH ROW
UPDATE actors
SET actors.spectacle_counter = actors.spectacle_counter + 1
WHERE actors.id = NEW.Actors_id
//
DELIMITER ;

DELIMITER //
CREATE TRIGGER update_bonus_salary
AFTER UPDATE
ON actors
FOR EACH ROW
BEGIN
SET @spectacle_counter = 0;
SELECT spectacle_counter
INTO @spectacle_counter
FROM actors
WHERE actors.id = NEW.id;
UPDATE contracts
SET contracts.bonus = 1000 * @spectacle_counter,
contracts.salary = contracts.base_salary + contracts.bonus
WHERE contracts.id = NEW.id;
END;
//
DELIMITER ;

SHOW TABLES;

SELECT * FROM actors;
SELECT * FROM additionaltable;
SELECT * FROM contracts;
SELECT * FROM roles;
SELECT * FROM shows;

INSERT INTO actors (first_name, last_name, awards)
VALUES ("Alex", "Kovalov", "Oscar"),
		("Danylo", "Zhukovskiy", ""),
		("Vlad", "Karkushevskiy", ""),
		("Liza", "Drelya", "Golden Globe"),
		("Artem", "Dikovskyi", "Israeli Academy of Film and Television");

SELECT * FROM actors;
SELECT * FROM contracts;

INSERT INTO contracts (base_salary, Actors_id)
VALUES (3000, 1),
		(4000, 2),
        (3500, 3),
        (6000, 4),
        (10000, 5);
        
INSERT INTO shows (title, budget)
VALUES ("Hamlet", 300000),
		("Long Day's Journey Into Night", 300000),
        ("Who's Afraid of Virginia Woolf?", 300000),
        ("Death of a Salesman", 300000),
        ("Oedipus Rex", 300000);
        
INSERT INTO roles (title)
VALUES ("Hamlet"),
		("Claudius"),
        ("Gertrude"),
        ("James Tyrone"),
        ("Mary Tyrone"),
        ("George"),
        ("Honey"),
        ("Linda Loman"),
        ("Oedipus"),
        ("Tiresias"),
        ("Jocasta");
        
INSERT INTO additionaltable (Roles_id, Shows_id, Actors_id)
VALUES (1, 1, 1),
		(2, 1, 3),
        (3, 1, 4),
        (4, 2, 2),
        (5, 2, 3),
        (6, 3, 1),
        (7, 3, 5),
        (8, 4, 1),
        (9, 5, 4),
        (10, 5, 5),
        (11, 5, 2);

SELECT * FROM actors;
SELECT * FROM additionaltable;
SELECT * FROM contracts;
SELECT * FROM roles;
SELECT * FROM shows;