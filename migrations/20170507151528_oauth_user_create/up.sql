-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    avatar VARCHAR NOT NULL,
)
