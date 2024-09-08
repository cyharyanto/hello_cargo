-- Your SQL goes here
CREATE TABLE users (
                       id VARCHAR PRIMARY KEY,
                       name VARCHAR NOT NULL,
                       email VARCHAR NOT NULL UNIQUE
);