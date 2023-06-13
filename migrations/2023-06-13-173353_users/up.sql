CREATE TABLE users
(
    id       SERIAL,
    name VARCHAR NOT NULL,
    username VARCHAR NOT NULL,
    email    VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    registration_date TIMESTAMP DEFAULT NOW(),
    CONSTRAINT pk_users PRIMARY KEY (id)
)