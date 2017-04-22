-- Your SQL goes here
CREATE TABLE users (
	id SERIAL PRIMARY KEY,
	email VARCHAR NOT NULL,
	username VARCHAR NOT NULL,
	name VARCHAR NOT NULL,

	avatar_const BOOLEAN NOT NULL DEFAULT 'f'
)

