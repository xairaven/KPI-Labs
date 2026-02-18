USE TransportSystem;

CREATE TABLE Employees
(
    id         INT IDENTITY (1, 1),
    first_name VARCHAR(30)      NOT NULL,
    patronymic VARCHAR(30),
    last_name  VARCHAR(30)      NOT NULL,
    email      VARCHAR(50),
    phone      VARCHAR(15),

    password   VARCHAR(64)      NOT NULL,
    salt       UNIQUEIDENTIFIER NOT NULL,

    CONSTRAINT PK_Employees PRIMARY KEY (id)
);

CREATE TABLE Roles
(
    id   INT IDENTITY (1, 1),
    role VARCHAR(30) NOT NULL

        CONSTRAINT PK_Roles PRIMARY KEY (id),
    CONSTRAINT UQ_Role UNIQUE (role)
);

CREATE TABLE JobTitles
(
    employee_id INT NOT NULL,
    role_id     INT NOT NULL
        CONSTRAINT DF_JobTitle_Role DEFAULT 1,

    CONSTRAINT FK_Employees_JobTitles FOREIGN KEY (employee_id) REFERENCES Employees (id)
        ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT FK_Roles_JobTitles FOREIGN KEY (role_id) REFERENCES Roles (id)
        ON UPDATE CASCADE ON DELETE SET DEFAULT
);

CREATE TABLE Orders
(
    id            INT IDENTITY (1, 1),
    client_id     INT          NOT NULL,
    title         VARCHAR(50)  NOT NULL,
    origin        VARCHAR(128) NOT NULL,
    destination   VARCHAR(128) NOT NULL,
    people_amount INT,
    weight        FLOAT,
    height        FLOAT,

    CONSTRAINT PK_Orders PRIMARY KEY (id),

    CONSTRAINT FK_Employees_Orders FOREIGN KEY (client_id) REFERENCES Employees (id)
        ON UPDATE NO ACTION ON DELETE NO ACTION
);

CREATE TABLE ShipmentStatuses
(
    id     INT IDENTITY (1, 1),
    status VARCHAR(30) NOT NULL,

    CONSTRAINT PK_ShipmentStatuses PRIMARY KEY (id),
    CONSTRAINT UQ_Status UNIQUE (status)
);

CREATE TABLE Shipments
(
    id         INT IDENTITY (1, 1),
    order_id   INT           NOT NULL,
    status_id  INT           NOT NULL,
    driver_id  INT           NOT NULL,
    start_time SMALLDATETIME NOT NULL DEFAULT GETDATE(),
    end_time   SMALLDATETIME NOT NULL,

    CONSTRAINT PK_Shipments PRIMARY KEY (id),
    CONSTRAINT UQ_Shipment_Order UNIQUE (order_id),

    CONSTRAINT FK_Orders_Shipments FOREIGN KEY (order_id) REFERENCES Orders (id)
        ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT FK_Statuses_Shipments FOREIGN KEY (status_id) REFERENCES ShipmentStatuses (id)
        ON UPDATE CASCADE ON DELETE NO ACTION,
    CONSTRAINT FK_Drivers_Shipments FOREIGN KEY (driver_id) REFERENCES Employees (id)
        ON UPDATE NO ACTION ON DELETE NO ACTION
);