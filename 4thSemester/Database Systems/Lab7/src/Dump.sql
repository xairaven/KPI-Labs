-- MySQL dump 10.13  Distrib 8.0.32, for Win64 (x86_64)
--
-- Host: localhost    Database: theateractors
-- ------------------------------------------------------
-- Server version	8.0.32

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `actors`
--

DROP TABLE IF EXISTS `actors`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `actors` (
  `id` int NOT NULL AUTO_INCREMENT,
  `first_name` varchar(45) NOT NULL,
  `last_name` varchar(45) NOT NULL,
  `hiring_date` datetime DEFAULT (curdate()),
  `years_of_experience` int DEFAULT (floor(((to_days(curdate()) - to_days(`hiring_date`)) / 365))),
  `awards` varchar(255) DEFAULT '',
  `spectacle_counter` int DEFAULT '0',
  `IsEmployeeSenior` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=57 DEFAULT CHARSET=utf8mb3;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `actors`
--

LOCK TABLES `actors` WRITE;
/*!40000 ALTER TABLE `actors` DISABLE KEYS */;
INSERT INTO `actors` VALUES (1,'Alex','Kovalov','2023-04-14 00:00:00',0,'Oscar',3,NULL),(4,'Liza','Drelya','2023-04-14 00:00:00',0,'Golden Globe',2,NULL),(5,'Artem','Dikovskyi','2023-04-14 00:00:00',0,'Israeli Academy of Film and Television',2,NULL),(6,'PENELOPE','GUINESS','2020-07-24 00:00:00',2,'',0,'-'),(7,'NICK','WAHLBERG','2021-01-01 00:00:00',2,'',0,'-'),(8,'ED','CHASE','2020-01-17 00:00:00',3,'',0,'-'),(9,'JENNIFER','DAVIS','2013-12-05 00:00:00',9,'',0,'+'),(10,'JOHNNY','LOLLOBRIGIDA','2016-03-12 00:00:00',7,'',0,'+'),(11,'BETTE','NICHOLSON','2015-11-26 00:00:00',7,'',0,'+'),(12,'GRACE','MOSTEL','2017-08-18 00:00:00',5,'',0,'-'),(13,'MATTHEW','JOHANSSON','2017-02-26 00:00:00',6,'',0,'+'),(14,'JOE','SWANK','2019-08-05 00:00:00',3,'',0,'-'),(16,'ZERO','CAGE','2014-01-05 00:00:00',9,'',0,'+'),(17,'KARL','BERRY','2017-02-10 00:00:00',6,'',0,'+'),(18,'UMA','WOOD','2020-03-28 00:00:00',3,'',0,'-'),(19,'VIVIEN','BERGEN','2016-07-28 00:00:00',6,'',0,'+'),(20,'CUBA','OLIVIER','2018-11-07 00:00:00',4,'',0,'-'),(21,'FRED','COSTNER','2021-03-30 00:00:00',2,'',0,'-'),(22,'HELEN','VOIGHT','2016-05-23 00:00:00',6,'',0,'+'),(23,'DAN','TORN','2014-12-02 00:00:00',8,'',0,'+'),(24,'BOB','FAWCETT','2022-02-15 00:00:00',1,'',0,'-');
/*!40000 ALTER TABLE `actors` ENABLE KEYS */;
UNLOCK TABLES;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_0900_ai_ci */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION' */ ;
DELIMITER ;;
/*!50003 CREATE*/ /*!50003 TRIGGER `update_bonus_salary` AFTER UPDATE ON `actors` FOR EACH ROW BEGIN
SET @spectacle_counter = 0;
SELECT spectacle_counter
INTO @spectacle_counter
FROM actors
WHERE actors.id = NEW.id;
UPDATE contracts
SET contracts.bonus = 1000 * @spectacle_counter,
contracts.salary = contracts.base_salary + contracts.bonus
WHERE contracts.id = NEW.id;
END */;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;

--
-- Table structure for table `additionaltable`
--

DROP TABLE IF EXISTS `additionaltable`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `additionaltable` (
  `Roles_id` int NOT NULL,
  `Shows_id` int NOT NULL,
  `Actors_id` int NOT NULL,
  KEY `fk_AdditionalTable_Roles1_idx` (`Roles_id`),
  KEY `fk_AdditionalTable_Shows1_idx` (`Shows_id`),
  KEY `fk_AdditionalTable_Actors1_idx` (`Actors_id`),
  CONSTRAINT `fk_AdditionalTable_Actors1` FOREIGN KEY (`Actors_id`) REFERENCES `actors` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `fk_AdditionalTable_Roles1` FOREIGN KEY (`Roles_id`) REFERENCES `roles` (`id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `fk_AdditionalTable_Shows1` FOREIGN KEY (`Shows_id`) REFERENCES `shows` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `additionaltable`
--

LOCK TABLES `additionaltable` WRITE;
/*!40000 ALTER TABLE `additionaltable` DISABLE KEYS */;
INSERT INTO `additionaltable` VALUES (1,1,1),(3,1,4),(6,3,1),(7,3,5),(8,4,1),(9,5,4),(10,5,5);
/*!40000 ALTER TABLE `additionaltable` ENABLE KEYS */;
UNLOCK TABLES;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_0900_ai_ci */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION' */ ;
DELIMITER ;;
/*!50003 CREATE*/ /*!50003 TRIGGER `update_played_roles_trigger` AFTER INSERT ON `additionaltable` FOR EACH ROW UPDATE actors
SET actors.spectacle_counter = actors.spectacle_counter + 1
WHERE actors.id = NEW.Actors_id */;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;

--
-- Table structure for table `contracts`
--

DROP TABLE IF EXISTS `contracts`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `contracts` (
  `id` int NOT NULL AUTO_INCREMENT,
  `base_salary` decimal(10,0) NOT NULL,
  `salary` decimal(10,0) NOT NULL DEFAULT ((`base_salary` + `bonus`)),
  `bonus` decimal(10,0) NOT NULL DEFAULT '0',
  `Actors_id` int NOT NULL,
  PRIMARY KEY (`id`),
  KEY `fk_Contracts_Actors_idx` (`Actors_id`),
  CONSTRAINT `fk_Contracts_Actors` FOREIGN KEY (`Actors_id`) REFERENCES `actors` (`id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=38 DEFAULT CHARSET=utf8mb3;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `contracts`
--

LOCK TABLES `contracts` WRITE;
/*!40000 ALTER TABLE `contracts` DISABLE KEYS */;
INSERT INTO `contracts` VALUES (1,3000,6000,3000,1),(4,6000,8000,2000,4),(5,10000,12000,2000,5),(6,2000,2000,0,6),(7,0,0,0,7),(8,5000,5000,0,8),(9,4000,4000,0,9),(10,3000,3000,0,10),(11,5000,5000,0,11),(12,8000,8000,0,12),(13,2000,2000,0,13),(14,5000,5000,0,14),(16,6000,6000,0,16),(17,1000,1000,0,17),(18,6000,6000,0,18),(19,6000,6000,0,19),(20,5000,5000,0,20),(21,4000,4000,0,21),(22,4000,4000,0,22),(23,1000,1000,0,23),(24,1000,1000,0,24);
/*!40000 ALTER TABLE `contracts` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `roles`
--

DROP TABLE IF EXISTS `roles`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `roles` (
  `id` int NOT NULL AUTO_INCREMENT,
  `title` varchar(50) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=27 DEFAULT CHARSET=utf8mb3;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `roles`
--

LOCK TABLES `roles` WRITE;
/*!40000 ALTER TABLE `roles` DISABLE KEYS */;
INSERT INTO `roles` VALUES (1,'Hamlet'),(2,'Claudius'),(3,'Gertrude'),(4,'James Tyrone'),(5,'Mary Tyrone'),(6,'George'),(7,'Honey'),(8,'Linda Loman'),(9,'Oedipus'),(10,'Tiresias'),(11,'Jocasta'),(12,'WESTMORELAND'),(13,'ADAM'),(14,'CARPENTER'),(15,'MILES'),(16,'MENDOZA'),(17,'PERKINS'),(18,'HOPKINS'),(19,'BAUGH'),(20,'JOY'),(21,'PHILLIPS'),(22,'SOTO'),(23,'GANNON'),(24,'HOWELL'),(25,'MURRELL'),(26,'VASQUEZ');
/*!40000 ALTER TABLE `roles` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `shows`
--

DROP TABLE IF EXISTS `shows`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `shows` (
  `id` int NOT NULL AUTO_INCREMENT,
  `title` varchar(255) NOT NULL,
  `budget` decimal(10,0) NOT NULL,
  `date` datetime DEFAULT (curdate()),
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=51 DEFAULT CHARSET=utf8mb3;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `shows`
--

LOCK TABLES `shows` WRITE;
/*!40000 ALTER TABLE `shows` DISABLE KEYS */;
INSERT INTO `shows` VALUES (1,'Hamlet',300000,'2023-04-14 00:00:00'),(2,'Long Day\'s Journey Into Night',300000,'2023-04-14 00:00:00'),(3,'Who\'s Afraid of Virginia Woolf?',300000,'2023-04-14 00:00:00'),(4,'Death of a Salesman',300000,'2023-04-14 00:00:00'),(5,'Oedipus Rex',300000,'2023-04-14 00:00:00'),(6,'MATRIX SNOWMAN',88557,'2023-11-27 00:00:00'),(7,'REEF SALUTE',30024,'2023-10-27 00:00:00'),(8,'FROST HEAD',284450,'2023-12-12 00:00:00'),(9,'CAT CONEHEADS',132177,'2024-03-17 00:00:00'),(10,'DOORS PRESIDENT',207536,'2024-04-04 00:00:00'),(11,'STEEL SANTA',241150,'2023-04-20 00:00:00'),(12,'GRAPES FURY',183141,'2024-01-30 00:00:00'),(13,'LOSER HUSTLER',192253,'2024-03-11 00:00:00'),(14,'THIEF PELICAN',11844,'2023-10-19 00:00:00'),(15,'IDENTITY LOVER',282460,'2024-01-23 00:00:00'),(16,'MIRACLE VIRTUAL',176769,'2023-11-05 00:00:00'),(17,'GHOST GROUNDHOG',36465,'2023-12-21 00:00:00'),(18,'MEMENTO ZOOLANDER',52017,'2023-08-27 00:00:00'),(19,'FERRIS MOTHER',150692,'2024-02-25 00:00:00'),(20,'HOTEL HAPPINESS',197410,'2023-08-17 00:00:00');
/*!40000 ALTER TABLE `shows` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Dumping events for database 'theateractors'
--

--
-- Dumping routines for database 'theateractors'
--
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2023-05-18 21:29:58
