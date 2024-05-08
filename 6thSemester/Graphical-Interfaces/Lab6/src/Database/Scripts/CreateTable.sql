CREATE TABLE Alarms
(
    id               BLOB PRIMARY KEY NOT NULL,
    title            NVARCHAR(100)    NOT NULL,
    is_alarm_enabled BOOLEAN          NOT NULL,
    datetime         DATETIME         NOT NULL
);